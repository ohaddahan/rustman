// TODO
// https://rust-cli.github.io/book/index.html

extern crate rustman_lib;
use std::str;
use rustman_lib::procfile::Procfile;
use rustman_lib::process::Process;
use std::process::Command;
use std::collections::HashMap;

//fn print(count: &mut i32, hash: &HashMap<String, String>, name: &str) {
//    *count += 1;
//    print!("{}: {} = {:#?}\n", name, count, hash);
//}

fn main() {
    //let mut count = 0;
    //let mut opt: HashMap<String, String> = HashMap::new();
    //let mut env: HashMap<String, String> = HashMap::new();

    let cmd = "rustman-lib/tests/test.sh";

    //let output = Command::new(cmd).
    //    output().
    //    expect("failed to execute process");
    //print!("output = {:#?}", output);

    //let output = Command::new("sh").
    //    arg("-c").
    //    arg(&cmd).
    //    spawn().
    //    expect("failed to execute process");
    //print!("output = {:#?}", output);

    let output = Command::new("sh").
        arg("-c").
        arg(&cmd).
        output().
        expect("failed to execute process");


    print!("output = {}", str::from_utf8(&output.stdout).unwrap());
    //let procfile = Procfile::new(String::from("rustman-lib/tests/Procfile1.test"));
    //println!("procfile:\n{}", procfile);
}
