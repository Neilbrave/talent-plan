#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    println!("Hello, world");
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    if let Some(configval) = m.value_of("config"){
	    match configval{
	    	"c1" => println!("config 1111"),
	    	"c2" => println!("config 2222"),
	    	"c3" => println!("config 3333"),
	    	_ => println!("what did you config?")
	    }
	} else {
		println!("--config is not assigned");
	}

	if let Some(inputval) = m.value_of("INPUT"){
	    println!("{:?}", inputval);		
	} else {
		println!("INPUT is not assigned");
	}
}
