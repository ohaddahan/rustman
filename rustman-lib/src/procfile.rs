/// Reads and writes Procfiles
///
/// A valid Procfile entry is captured by this regex:
///
///   /^([A-Za-z0-9_]+):\s*(.+)$/
///
/// All other lines are ignored.
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Index;
use std::fmt;

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
    entries: HashMap<String, Entry>,
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
    fn load(&mut self) {
        self.entries.clear();
        self.parse()
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
                let entry = Entry::new(line.to_string(), (&cap[1]).to_string(), (&cap[2]).to_string());
                self.entries.insert(entry.name.clone(), entry);
            }
        }
    }

    pub fn new(filename: String) -> Procfile {
        let mut procfile = Procfile {
            filename,
            entries: HashMap::new(),
        };
        procfile.parse();
        procfile
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_entry_constructor() {
        let entry = Entry::new("hello".to_string(), "world".to_string(), "rust".to_string());
        assert_eq!(entry.line, "hello".to_string());
        assert_eq!(entry.name, "world".to_string());
        assert_eq!(entry.command, "rust".to_string());
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
}
