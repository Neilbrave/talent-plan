#[macro_use]
extern crate clap;
use clap::App;
use std::process::exit;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("../kvs.yml");
    let m = App::from_yaml(yaml).get_matches();

    // get_matches 正则花匹配
    match m.subcommand() {
        ("set", Some(_)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(_)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_)) => {
           eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}