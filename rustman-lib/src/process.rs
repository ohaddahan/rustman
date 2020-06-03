use std::io::prelude::*;
use std::str;
use std::error::Error;
use std::process::{Command, Stdio, Child, Output};
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

    pub fn run(&self, options: Option<HashMap<String, String>>) -> Child {
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
            envs(&env).
            stdout(Stdio::piped()).
            spawn().
            expect("failed to execute process");
        self.chdir(path.into_os_string().into_string().unwrap());
        output
    }

    pub fn exec(&mut self, options: Option<HashMap<String, String>>) -> String {
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
        let output = Command::new("sh").
            arg("-c").
            arg(cmd).
            output().
            expect("failed to execute process");
        str::from_utf8(&output.stdout).unwrap().to_string()
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
    static ENV_BIN:  &'static str = "tests/env.sh";
    static ECHO_BIN: &'static str = "tests/echo.sh";
    static UTF8_BIN: &'static str = "tests/utf8.sh";

    fn run(process: Process, options: Option<HashMap<String, String>>) -> String {
        let mut child = process.run(options);
        let output = child.wait().unwrap();
        let mut s = String::new();
        match child.stdout.unwrap().read_to_string(&mut s) {
            Err(e) => panic!("couldn't read wc stdout: {}", e),
            Ok(_) => s
        }
    }

    #[test]
    fn test_cwd() {
        //TODO
    }
    #[test]
    fn test_chdir() {
        //TODO
    }
    #[test]
    fn test_exec() {
        //TODO
    }
    #[test]
    fn test_run() {
        //TODO
    }
    #[test]
    fn test_expanded_command() {
        //TODO
    }

    #[test]
    fn test_runs_the_process() {
        let process = Process::new(TEST_BIN.to_string(), None, None, None);
        assert_eq!("testing\n", run(process, None));

    }

    #[test]
    fn test_can_set_environment() {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("FOO".to_string(), "bar".to_string());
        let process = Process::new(format!("{} FOO", ENV_BIN), None, None, Some(env));
        assert_eq!("bar\n", run(process, None));
    }

    #[test]
    fn test_can_handle_env_vars_in_the_command() {
        let hello = String::from("Olá");

        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("FOO".to_string(), "bar".to_string());
        let process = Process::new(format!("{} $FOO", ECHO_BIN), None, None, Some(env));
        assert_eq!("bar\n", run(process, None));
    }

    #[test]
    fn test_can_handle_per_run_env_vars_in_the_command() {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("FOO".to_string(), "bar".to_string());
        let process = Process::new(format!("{} $FOO", ECHO_BIN), None, None, None);
        assert_eq!("bar\n", run(process, Some(env)));
    }

    //TODO
    //#[test]
    //fn test_should_output_utf8_properly() {
    //    let process = Process::new(UTF8_BIN.to_string(), None, None, None);
    //    assert_eq!(str::from_utf8(b"\\xE2").unwrap(), run(process, None));
    //}

    #[test]
    fn test_can_expand_env_in_the_command() {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("FOO".to_string(), "bar".to_string());
        let process = Process::new("command $FOO $BAR".to_string(), None, None, Some(env));
        assert_eq!("command bar $BAR", process.expanded_command(None));
    }

    #[test]
    fn test_can_expand_extra_env_in_the_command() {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("FOO".to_string(), "bar".to_string());
        let process = Process::new("command $FOO $BAR".to_string(), None, None, Some(env));
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("BAR".to_string(), "qux".to_string());
        assert_eq!("command bar qux", process.expanded_command(Some(&env)));
    }

    #[test]
    fn test_can_execute() {
        let mut process = Process::new(TEST_BIN.to_string(), None, None, None);
        assert_eq!("testing\n", process.exec(None));
    }

    #[test]
    fn test_can_execute_with_env() {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("FOO".to_string(), "bar".to_string());
        let mut process = Process::new(format!("{} FOO", ENV_BIN), None, None, None);
        assert_eq!("bar\n", process.exec(Some(env)));
    }
}