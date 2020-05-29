use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::Index;
use std::fs::File;
use std::io::prelude::*;


lazy_static! {
    static ref RE: Regex = Regex::new(r"^([A-Za-z0-9_-]+):\s*(.+)$").expect("Cannot build regexp");
}

#[derive(Debug)]
pub struct Entry {
    line: String,
    name: String,
    command: String,
}

impl Entry {
    pub fn new(line: String, name: String, command: String) -> Entry {
        Entry {
            line,
            name,
            command,
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.command)
    }
}

#[derive(Debug)]
pub struct Procfile {
    entries: BTreeMap<String, Entry>,
}

impl Index<String> for Procfile {
    type Output = Entry;
    fn index(&self, i: String) -> &Entry {
        &self.entries[&i]
    }
}

impl fmt::Display for Procfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let newline = "\n";
        let mut count = 0;

        for (_entry_name, entry) in self.entries.iter() {
            if count > 0 {
                f.write_str(newline)?
            } else {
                count += 1;
            }
            f.write_str(&entry.to_string())?;
        }
        Ok(())
    }
}

impl<'a> Procfile {
    fn load(&mut self, filename: Option<&str>) {
        self.entries.clear();
        self.parse(filename);
    }

    fn guard_filename(&self, filename: Option<&'a str>) -> &'a str {
        match filename {
            Some(i) => {
                i
            },
            _ => {
                panic!("Cannot parse when filename is None");
            }
        }
    }

    fn delete(&mut self, name: &str) {
        self.entries.remove(name);
    }

    fn save(&self, filename: Option<&str>) -> std::io::Result<()> {
        let file_to_save = self.guard_filename(filename);
        let mut file = File::create(file_to_save)?;
        file.write_all(self.to_string().as_bytes())?;
        file.sync_all()?;
        Ok(())
    }

    fn parse(&mut self, filename: Option<&str>) {
        let file_to_parse = self.guard_filename(filename);
        let data = match std::fs::read_to_string(file_to_parse) {
            Ok(string) => string,
            Err(error) => {
                panic!("Error {:?}", error);
            }
        };

        for line in data.replace("\r\n", "\n").split('\n') {
            for cap in RE.captures_iter(line) {
                let entry = Entry::new(
                    line.to_string(),
                    (&cap[1]).to_string(),
                    (&cap[2]).to_string(),
                );
                self.entries.insert(entry.name.clone(), entry);
            }
        }
    }

    pub fn new(filename: Option<&str>) -> Procfile {
        let mut procfile = Procfile {
            entries: BTreeMap::new(),
        };
        match filename {
            Some(_) => {
                procfile.parse(filename);
                procfile
            },
            None => procfile
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::path::Path;

    static PROCFILE_IN_PATH: &'static str = "tests/Procfile";
    static PROCFILE_OUT_PATH: &'static str = "tests/Procfile.out";
    static PROCFILE_WRITE_PROCFILE: &'static str = "tests/Procfile.tmp";

    struct TmpFile {
        filename: String
    }


    impl Drop for TmpFile {
        fn drop(&mut self) {
            TmpFile::delete_file(self.filename.as_str());
        }
    }

    impl TmpFile {
        fn delete_file(filename: &str) {
            if Path::new(filename).exists() {
                std::fs::remove_file(filename);
            }
        }

        fn write_procfile(procfile: Option<&str>, alpha_env: Option<&str>) -> TmpFile {
            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(0, 1000000);
            let filename = match procfile {
                Some(i) => i,
                None => PROCFILE_WRITE_PROCFILE
            };
            let final_filename = format!("{}.{}", filename, random_number);
            let mut file = File::create(final_filename.as_str()).
                expect("write_procfile failed creating file");
            let alpha = match alpha_env {
                Some(i) => i,
                None => ""
            };
            file.write_all(format!("alpha: ./alpha{}\n", alpha).as_bytes());
            file.write_all(format!("bravo:\t./bravo\n").as_bytes());
            file.write_all(format!("foo_bar:\t./foo_bar\n").as_bytes());
            file.write_all(format!("foo-bar:\t./foo-bar\n").as_bytes());
            file.write_all(format!("# baz:\t./baz\n").as_bytes());
            file.sync_all();
            let tmpfile = TmpFile { filename: final_filename.to_string() };
            tmpfile
        }
    }

    #[test]
    fn test_entry_constructor() {
        let entry = Entry::new(
            String::from("hello"),
            String::from("world"),
            String::from("rust"),
        );
        assert_eq!(entry.line, String::from("hello"));
        assert_eq!(entry.name, String::from("world"));
        assert_eq!(entry.command, String::from("rust"));
    }

    #[test]
    fn test_regexp_creation() {
        match Regex::new(RE.as_str()) {
            Result::Err(_) => panic!("Failed building regexp"),
            _ => {}
        }
    }

    #[test]
    fn test_procfile_entry() {
        let line = String::from("web: rails server");
        for cap in RE.captures_iter(&line) {
            let entry = Entry::new(
                line.to_string(),
                (&cap[1]).to_string(),
                (&cap[2]).to_string(),
            );
            assert_eq!(entry.line, line);
            assert_eq!(entry.name, String::from("web"));
            assert_eq!(entry.command, String::from("rails server"));
        }
    }

    #[test]
    fn test_procfile_falty_entry() {
        let line = String::from("web: ");
        assert_eq!(1, RE.captures_iter(&line).count());
        for cap in RE.captures_iter(&line) {
            let entry = Entry::new(
                line.to_string(),
                (&cap[1]).to_string(),
                (&cap[2]).to_string(),
            );
            assert_eq!(entry.line, line);
            assert_eq!(entry.name, String::from("web"));
        }
    }

    #[test]
    fn test_parse() {
        let procfile1 = std::fs::read_to_string(PROCFILE_IN_PATH)
            .expect("test_parse failed reading procfile1_path");
        let procfile = Procfile::new(Some(PROCFILE_IN_PATH));
        assert_eq!(procfile.to_string(), procfile1.trim().to_string());
    }

    #[test]
    fn test_procfile_save() {
        TmpFile::delete_file(PROCFILE_OUT_PATH);

        let procfile1 = std::fs::read_to_string(PROCFILE_IN_PATH)
            .expect("test_procfile_save failed reading procfile1_path");
        let procfile = Procfile::new(Some(PROCFILE_IN_PATH));
        assert_eq!(procfile.to_string(), procfile1.trim().to_string());

        procfile.save(Some(PROCFILE_OUT_PATH)).expect("Save failed");

        let procfile2 = std::fs::read_to_string(PROCFILE_OUT_PATH)
            .expect("test_procfile_save failed reading procfile2_path");
        let procfile = Procfile::new(Some(PROCFILE_OUT_PATH));
        assert_eq!(procfile.to_string(), procfile2.trim().to_string());

        TmpFile::delete_file(PROCFILE_OUT_PATH);
    }

    #[test]
    fn test_delete() {
        let mut procfile = Procfile::new(Some(PROCFILE_IN_PATH));
        assert_eq!(5, procfile.entries.len());
        procfile.delete(&procfile.entries.keys().next().unwrap().clone());
        assert_eq!(4, procfile.entries.len());

    }
    // Tests from https://github.com/ddollar/foreman/blob/master/spec/foreman/procfile_spec.rb
    #[test]
    fn test_can_load_from_a_file() {
        let tmpfile = TmpFile::write_procfile(Some(PROCFILE_WRITE_PROCFILE), None);
        let mut procfile = Procfile::new(None);
        procfile.load(Some(tmpfile.filename.as_str()));
        assert_eq!("./alpha", procfile["alpha".to_string()].command);
        assert_eq!("./bravo", procfile["bravo".to_string()].command);
    }
    #[test]
    fn test_loads_a_passed_in_procfile() {
        let tmpfile = TmpFile::write_procfile(Some(PROCFILE_WRITE_PROCFILE), None);
        let procfile = Procfile::new(Some(tmpfile.filename.as_str()));
        assert_eq!("./alpha", procfile["alpha".to_string()].command);
        assert_eq!("./bravo", procfile["bravo".to_string()].command);
        assert_eq!("./foo-bar", procfile["foo-bar".to_string()].command);
        assert_eq!("./foo_bar", procfile["foo_bar".to_string()].command);
    }
    #[test]
    fn test_it_only_creates_procfile_entries_for_lines_matching_regex() {
        let tmpfile = TmpFile::write_procfile(Some(PROCFILE_WRITE_PROCFILE), None);
        let procfile = Procfile::new(Some(tmpfile.filename.as_str()));
        let ref_keys = vec!["alpha", "bravo", "foo-bar", "foo_bar"];
        let test_keys: Vec<&str> = procfile.entries.
            keys().
            map(|i| i.as_str()).
            collect();
        assert_eq!(ref_keys, test_keys);
    }

    #[test]
    fn test_returns_nil_when_attempting_to_retrieve_an_non_existing_entry() {
//        write_procfile
//        procfile = Foreman::Procfile.new("Procfile")
//        expect(procfile["unicorn"]).to eq(nil)
    }


    #[test]
    fn test_can_have_a_process_appended_to_it() {
        //subject["charlie"] = "./charlie"
        //expect(subject["charlie"]).to eq("./charlie")
    }


    #[test]
    fn test_can_write_to_a_string() {
        //it "can write to a string" do
        //subject["foo"] = "./foo"
        //subject["bar"] = "./bar"
        //expect(subject.to_s).to eq("foo: ./foo\nbar: ./bar")
        //end
    }

    #[test]
    fn test_can_write_to_a_file() {
        // subject["foo"] = "./foo"
        // subject["bar"] = "./bar"
        // Dir.mkdir('/tmp')
        // subject.save "/tmp/proc"
        // expect(File.read("/tmp/proc")).to eq("foo: ./foo\nbar: ./bar\n")
        // end
    }
}