use std::io;
use std::{env, fs, fmt};
use std::path::Path;
use std::process::{Command, Stdio};


// pub mod terminal;

// fn current_dir() -> io::Result<()>{
// 	let current_dir = match env::current_dir() {
//     // println!("{:?}", current_dir);

//     	// Ok(current_dir) => println!("{:?}", current_dir),
//     	Ok(current_dir) => current_dir,
//     	Err(e) => return Err(e),
// 	};

// 	println!("{:?}", current_dir);
// 	// return Ok(current_dir)
// 	Ok(())
// }

#[shell]
fn change_dir(home_dir: PathBuf, current_dir: PathBuf) {
	println!("{:?}", home_dir);
}

fn search(path: String) {
	// println!("hello world");
}

fn main() {
	// let args: Vec<String> = env::args().collect();
	// println!("{:?}", args);

	// let path = &args[1];

	let home_dir = env::home_dir().unwrap();
	println!("{:?}", home_dir.to_str());

	let current_dir = env::current_dir().unwrap();
	println!("{:?}", current_dir.to_str());

	// search(path.to_string());
// 
	let path1 = Path::new("/home/saurabh/Desktop");
	env::set_current_dir(path1).is_ok();
	println!("Successfully changed working directory to {}!", path1.display());


	change_dir(&home_dir, &current_dir);
	// Command::new("ls")
	// 	.current_dir("/home/saurabh/")
 //        .spawn()
 //        .expect("command failed to start");

 	// let path = current_dir();
 	// println!("{:?}", path);
}


