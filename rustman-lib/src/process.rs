use std::io::prelude::*;
use std::error::Error;
use std::process::{Command, Stdio};
use std::env;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Process {
    command: String,
    cwd: Option<String>,
    output: Option<String>,
    env: HashMap<String, String>,
}

impl Process {
    pub fn new(command: String, cwd: Option<String>, output: Option<String>, env: Option<HashMap<String, String>>) -> Process {
        Process {
            command,
            cwd,
            output,
            env: match env {
                Some(i) => i,
                None => HashMap::new()
            }
        }
    }

    pub fn expanded_command(&self, custom_env: Option<&HashMap<String, String>>) -> String {
        let mut out_expanded_command = self.command.clone();
        let mut env = self.env.clone();
        match custom_env {
            Some(i) => {
                for (key, val) in i.iter() {
                    env.insert(key.to_string(), val.to_string());
                }
            },
            None => ()
        }
        for (key, val) in env.iter() {
            out_expanded_command = out_expanded_command.replace(&format!("${}", key), val);
        }
        out_expanded_command
    }

    pub fn run(&self, options: Option<HashMap<String, String>>) -> String {
        let mut env = self.env.clone();
        match options {
            Some(i) => env.extend(i),
            None => ()
        }
        let path = env::current_dir().unwrap();
        self.chdir(self.cwd());
        let cmd = self.expanded_command(Some(&env));
        let output = Command::new("sh").
            arg("-c").
            arg(cmd).
            stdout(Stdio::piped()).
            spawn().
            expect("failed to execute process");
        self.chdir(path.into_os_string().into_string().unwrap());
        let mut s = String::new();
        match output.stdout.unwrap().read_to_string(&mut s) {
            Err(e) => panic!("couldn't read wc stdout: {}", e),
            Ok(_) => s
        }
    }

    pub fn exec(&mut self, options: Option<HashMap<String, String>>) {
        let mut env = self.env.clone();
        match options {
            Some(i) => env.extend(i),
            None => ()
        }
        for (key, val) in env.iter() {
            env::set_var(key, val);
        }
        self.chdir(self.cwd());
        let cmd = self.expanded_command(Some(&self.env));
        let _output = Command::new("sh").
            arg("-c").
            arg(cmd).
            output().
            expect("failed to execute process");
    }

    fn chdir(&self, in_cwd: String) {
        assert!(env::set_current_dir(&in_cwd).is_ok());
        println!("Successfully changed working directory to {}!", in_cwd);
    }

    pub fn cwd(&self) -> String {
        let env_cwd = match self.env.get("cwd") {
            Some(i) => i,
            None => "."
        };
        Path::new(env_cwd).
            canonicalize().
            unwrap().
            into_os_string().
            into_string().
            unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST_BIN: &'static str = "tests/test.sh";

    #[test]
    fn test_runs_the_process() {
        let process = Process::new(TEST_BIN.to_string(), None, None, None);
        assert_eq!("testing\n", process.run(None));

    }
    //process = Foreman::Process.new(resource_path("bin/test"))
    //expect(run(process)).to eq("testing\n")
    //end

    #[test]
    fn test_cwd() {}
    #[test]
    fn test_chdir() {}
    #[test]
    fn test_exec() {}
    #[test]
    fn test_run() {}
    #[test]
    fn test_expanded_command() {}
}