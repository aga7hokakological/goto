use std::io;
use std::{env, fs, fmt};
use std::fs::canonicalize;
use std::path::{self, Path, PathBuf};
use std::process::{Command, Stdio};

use walkdir::WalkDir;

extern crate skim;
use skim::prelude::*;

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

// #[shell]
// fn change_dir(home_dir: PathBuf, current_dir: PathBuf) {
// 	println!("{:?}", home_dir);
// }

// fn search(path: String) -> Result<(), io::Error> {
// 	for entry in WalkDir::new(&path) {
//     	println!("{}", entry.path().display());
// 	}
// }

use path_clean::PathClean;

fn absolute_path<P>(path: P) -> io::Result<PathBuf>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }.clean();

    println!("{:?}", absolute_path);
    Ok(absolute_path)
}

fn create_path(home_dir: &std::path::PathBuf, arg_path: &std::string::String) -> io::Result<()> {
	for entry in fs::read_dir(home_dir)? {
		let dir = entry?;
		println!("{:?}", dir.path());
	}
	Ok(())
}

fn main() {
	let args: Vec<String> = env::args().collect();
	// println!("{:?}", args);

	let arg_path = &args[1];

	let home_dir = env::home_dir().unwrap();
	println!("{:?}", home_dir.to_str());

	let current_dir = env::current_dir().unwrap();
	println!("{:?}", current_dir.to_str());

	// search(&path);

	// let goto_dir = PathBuf::from(&path);
	// println!("{:?}", fs::canonicalize(&goto_dir));

	let mut p = PathBuf::from("");
	p.push(&path::MAIN_SEPARATOR.to_string());
	p.push(&arg_path);
	println!("{}", p.to_str().unwrap());

	// println!("{:?}", canonicalize(&p));

	absolute_path(&p);

	create_path(&home_dir, arg_path);

	let path1 = Path::new(&p);
	env::set_current_dir(&path1).is_ok();
	println!("Successfully changed working directory to {}!", path1.display());

	// change_dir(&home_dir, &current_dir);
	Command::new("ls")
		.current_dir(&path1)
        .spawn()
        .expect("command failed to start");

 	// let path = current_dir();
 	// println!("{:?}", path);
}


