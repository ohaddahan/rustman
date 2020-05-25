/// Reads and writes Procfiles
///
/// A valid Procfile entry is captured by this regex:
///
///   /^([A-Za-z0-9_]+):\s*(.+)$/
///
/// All other lines are ignored.
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::ops::Index;
use std::path::Path;

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
    filename: String,
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

impl Procfile {
    /*
    fn load(&mut self) {
        self.entries.clear();
        self.parse()
    }
    */

    fn delete(&mut self, name: &String) {
        self.entries.remove(name);
    }

    fn save(&self, filename: Option<&str>) -> std::io::Result<()> {
        let output_filename = match filename {
            Some(_) => filename.unwrap(),
            None => &self.filename
        };
        let mut file = File::create(output_filename)?;
        file.write_all(self.to_string().as_bytes())?;
        file.sync_all()?;
        Ok(())
    }

    fn parse(&mut self) {
        let data = match std::fs::read_to_string(&self.filename) {
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

    pub fn new(filename: String) -> Procfile {
        let mut procfile = Procfile {
            filename,
            entries: BTreeMap::new(),
        };
        procfile.parse();
        procfile
    }
}

#[cfg(test)]
mod tests {
    static PROCFILE_IN_PATH: &'static str = "tests/Procfile.in.test";
    static PROCFILE_OUT_PATH: &'static str = "tests/Procfile.out.test";
    use super::*;
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
        let procfile1_path = String::from(PROCFILE_IN_PATH);
        let procfile1 = std::fs::read_to_string(&procfile1_path)
            .expect("test_parse failed reading procfile1_path");
        let procfile = Procfile::new(procfile1_path);
        assert_eq!(procfile.to_string(), procfile1.trim().to_string());
    }

    #[test]
    fn test_procfile_save() {
        let procfile1_path = String::from(PROCFILE_IN_PATH);
        let procfile2_path = String::from(PROCFILE_OUT_PATH);;

        if Path::new(&procfile2_path).exists() {
            std::fs::remove_file(&procfile2_path);
        }

        let procfile1 = std::fs::read_to_string(&procfile1_path)
            .expect("test_procfile_save failed reading procfile1_path");
        let procfile = Procfile::new(procfile1_path);
        assert_eq!(procfile.to_string(), procfile1.trim().to_string());

        procfile.save(Some(&procfile2_path[..])).expect("Save failed");

        let procfile2 = std::fs::read_to_string(&procfile2_path)
            .expect("test_procfile_save failed reading procfile2_path");
        let procfile = Procfile::new(procfile2_path.clone());
        assert_eq!(procfile.to_string(), procfile2.trim().to_string());

        if Path::new(&procfile2_path).exists() {
            std::fs::remove_file(&procfile2_path);
        }
    }

    #[test]
    fn test_delete() {
        let procfile1_path = String::from(PROCFILE_IN_PATH);
        let procfile1 = std::fs::read_to_string(&procfile1_path)
            .expect("test_parse failed reading procfile1_path");
        let mut procfile = Procfile::new(procfile1_path);
        assert_eq!(2, procfile.entries.len());
        procfile.delete(&String::from("web"));
        assert_eq!(1, procfile.entries.len());

    }
}
