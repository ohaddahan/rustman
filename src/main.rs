extern crate rustman_lib;
use rustman_lib::procfile::Procfile;
use std::process::Command;
use std::collections::HashMap;

fn print(count: &mut i32, hash: &HashMap<String, String>, name: &str) {
    *count += 1;
    print!("{}: {} = {:#?}\n", name, count, hash);
}

fn main() {
    let mut count = 0;
    let mut opt: HashMap<String, String> = HashMap::new();
    let mut env: HashMap<String, String> = HashMap::new();

    print(&mut count, &opt, "opt");
    print(&mut count, &env, "env");
    opt.insert("opt1".to_string(), "roar1".to_string());
    print(&mut count, &opt, "opt");
    print(&mut count, &env, "env");
    env.insert("env1".to_string(), "val1".to_string());
    print(&mut count, &env, "env");
    env.extend(opt);
//    print(&mut count, &opt, "opt");
    print(&mut count, &env, "env");

    let cmd = std::fs::read_to_string("cmd.sh").unwrap();
    print!("\n-----------\n");
    print!("cmd = {}", cmd);
    print!("\n-----------\n");
    let output = Command::new("sh").
        arg("-c").
        arg(&cmd).
        spawn().
        expect("failed to execute process");
    print!("output = {:#?}", output);

    let output = Command::new("sh").
        arg("-c").
        arg(&cmd).
        output().
        expect("failed to execute process");


    print!("output = {:#?}", output);
    //let procfile = Procfile::new(String::from("rustman-lib/tests/Procfile1.test"));
    //println!("procfile:\n{}", procfile);
}
