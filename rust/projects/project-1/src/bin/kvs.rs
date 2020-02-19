#[macro_use]
extern crate clap;
use clap::App;
use std::process::exit;

<<<<<<< HEAD

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
=======
fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    let _v = match m.occurrences_of("V"){
        _ => println!("0.1.0")
    };

    match m.subcommand() {
        ("set", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(_matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
>>>>>>> 8002722a9a5e30fe3304b2f75b357b5af428a8b6
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
<<<<<<< HEAD

=======
>>>>>>> 8002722a9a5e30fe3304b2f75b357b5af428a8b6
}