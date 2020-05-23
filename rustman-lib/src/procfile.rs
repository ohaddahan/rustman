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
            for cap in re.captures_iter("ss") {
                self.entries.push(Entry::new(
                    line.to_string(),
                    (&cap[0]).to_string(),
                    (&cap[1]).to_string(),
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
