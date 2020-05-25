extern crate rustman_lib;
use rustman_lib::procfile::Procfile;
use std::process::Command;
fn main() {
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
