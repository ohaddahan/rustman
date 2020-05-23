extern crate rustman_lib;
use rustman_lib::procfile::Procfile;

fn main() {
    let procfile = Procfile::new(String::from("rustman-lib/tests/Procfile1.test"));
    println!("procfile:\n{}", procfile);
}
