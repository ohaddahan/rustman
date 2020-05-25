use std::process::Command;
use std::env;
use std::path::Path;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Process {
    command: String,
    cwd: String,
    output: String,
    env: HashMap<String, String>,
}

impl Process {
    pub fn new(command: String, cwd: Option<String>, output: Option<String>, env: Option<HashMap<String, String>>) -> Process {
        Process { command, cwd: cwd.unwrap() , output: output.unwrap() , env: env.unwrap() }
    }

    pub fn expanded_command(&self, custom_env: Option<&HashMap<String, String>>) -> String {
        let mut out_expanded_command = self.command.clone();
        let mut env = self.env.clone();
        for (key, val) in custom_env.unwrap().iter() {
            env.insert(key.to_string(), val.to_string());
        }
        for (key, val) in env.iter() {
            out_expanded_command = out_expanded_command.replace(&format!("${}", key), val);
        }
        out_expanded_command
    }

    pub fn run(&self, options: Option<HashMap<String, String>>) {
        let mut env = self.env.clone();
        if let Some(e) = options { env.extend(e) }
        let path = env::current_dir().unwrap();
        self.chdir(self.cwd());
        let cmd = self.expanded_command(Some(&env));
        let _output = Command::new("sh").
            arg("-c").
            arg(cmd).
            spawn().
            expect("failed to execute process");
        self.chdir(path.into_os_string().into_string().unwrap());
    }

    pub fn exec(&mut self, options: Option<HashMap<String, String>>) {
        if let Some(e) = options { self.env.extend(e) }
        for (key, val) in self.env.iter() { env::set_var(key, val); }
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