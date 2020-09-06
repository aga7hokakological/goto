// use std::io;
use std::env;

#[macro_use] extern crate lazy_static;

// pub mod terminal;
pub mod dir;


fn main() {
	let args: Vec<String> = env::args().collect();
	// println!("{:?}", args);

	let arg_path = &args[1];

	let home_dir = dirs::home_dir().unwrap();
	println!("{:?}", home_dir.to_str());

	let current_dir = env::current_dir().unwrap();
	println!("{:?}", current_dir.to_str());


	// let mut p = PathBuf::from("");
	// p.push(&path::MAIN_SEPARATOR.to_string());
	// p.push(&arg_path);
	// println!("{}", p.to_str().unwrap());

	dir::recursive_file_traversal(&home_dir, arg_path);

	// dir::change_dir(new_dir);
}


