extern crate rustman_lib;
use rustman_lib::procfile::Procfile;


fn main() {
    let procfile = Procfile::new("rustman-lib/tests/Procfile.test".to_string());
    println!("procfile = {:#?}", procfile);
}
