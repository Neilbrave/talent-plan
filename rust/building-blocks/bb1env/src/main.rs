fn main() {
    println!("Hello, world!");
	use std::env;

	let key = "HOME";
	match env::var_os(key) {
	    Some(val) => println!("{}: {:?}", key, val),
	    None => println!("{} is not defined in the environment.", key)
	}
}
