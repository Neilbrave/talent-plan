use std::env;

enum ErrTypes{
	Err111,
//	Err222,
}

fn getargs(args: Vec<String>) -> Result<String, ErrTypes>{
    match args.get(1) {
    	Some(_v) => Ok(_v.to_string()),
    	None => Err(ErrTypes::Err111)
    }
}

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let val=getargs(args);
    match val{
    	Ok(_v) => println!("OK, val == {:?}", _v),
    	Err(_e) => println!("Error!!!!!")
    }
}
