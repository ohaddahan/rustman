/// Reads and writes Procfiles
///
/// A valid Procfile entry is captured by this regex:
///
///   /^([A-Za-z0-9_]+):\s*(.+)$/
///
/// All other lines are ignored.
use regex::Regex;

#[derive(Debug)]
struct Entry {
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

#[derive(Debug)]
pub struct Procfile {
    filename: String,
    entries: Vec<Entry>,
}

impl Procfile {
    fn parse(&mut self) {
        let data = match std::fs::read_to_string(&self.filename) {
            Ok(string) => string,
            Err(error) => {
                panic!("Error {:?}", error);
            }
        };

        let re = Regex::new(r"^([A-Za-z0-9_-]+):\s*(.+)$").expect("Cannot build regexp");
        for line in data.replace("\r\n", "\n").split('\n') {
            for cap in re.captures_iter(line) {
                self.entries.push(Entry::new(
                    line.to_string(),
                    (&cap[1]).to_string(),
                    (&cap[2]).to_string(),
                ));
            }
        }
    }

    pub fn new(filename: String) -> Procfile {
        let mut procfile = Procfile {
            filename,
            entries: Vec::new(),
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
        let re = Regex::new(r"^([A-Za-z0-9_-]+):\s*(.+)$");
        match re {
            Result::Err(_) => panic!("Failed building regexp"),
            _ => {}
        }
    }

    #[test]
    fn test_procfile_entry() {
        let line = "web: rails server".to_string();
        let re = Regex::new(r"^([A-Za-z0-9_-]+):\s*(.+)$").expect("Cannot build regexp");
        for cap in re.captures_iter(&line) {
            let entry = Entry::new(
                line.to_string(),
                (&cap[1]).to_string(),
                (&cap[2]).to_string(),
            );
            assert_eq!(entry.line, line);
            assert_eq!(entry.name, "web".to_string());
            assert_eq!(entry.command, "rails server".to_string());
        }
    }
}
