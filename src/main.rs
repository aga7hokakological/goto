use std::io;
use std::{env, fs, fmt};
use std::fs::canonicalize;
use std::path::{self, Path, PathBuf};
use std::process::{Command, Stdio};

use walkdir::WalkDir;

extern crate regex;
use regex::Regex;

extern crate skim;
use skim::prelude::*;

// pub mod terminal;


// fn create_path(home_dir: &std::path::PathBuf, arg_path: &std::string::String) -> io::Result<()> {
// 	for entry in fs::read_dir(home_dir)? {
// 		let dir = entry?;
// 		println!("{:?}", dir.path());
// 	}
// 	Ok(())
// }

fn recursive_file_traversal<P>(home_dir: P, arg_path: &std::string::String)
where
    P: AsRef<Path> {

    let re = Regex::new(r"(\w+)$").unwrap();

    for entry in fs::read_dir(home_dir).unwrap(){
        let path_buffer =  entry.unwrap().path();
        let file_path= path_buffer.display().to_string();        
        if !file_path.contains("$")
        {
            let attrib_result = fs::metadata(file_path.clone());
            if attrib_result.is_ok()
            {
                let attributes =  attrib_result.unwrap();

                if attributes.is_dir() {

                    let new_directory = file_path.clone();
                    // println!("{:?}", new_directory);
                    let match_path = re.find(&new_directory).unwrap();
                    // println!("{:?}", match_path);

                    let start = match_path.start();
                    let end = match_path.end();

                    if &new_directory[start..end] == arg_path.as_str() {
                    	println!("{:?}", new_directory);
                    } else {
                    	recursive_file_traversal(new_directory, arg_path);
                    }
                }
                // else if attributes.is_file()
                // {
                //     println!("{}", file_path.clone());
                // }
            }
        }
    }
    // return new_directory
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

	// let mut p = PathBuf::from("");
	// p.push(&path::MAIN_SEPARATOR.to_string());
	// p.push(&arg_path);
	// println!("{}", p.to_str().unwrap());

	// println!("{:?}", canonicalize(&p));

	// absolute_path(&p);

	recursive_file_traversal(&home_dir, arg_path);

	// println!("{:?}", path);

	// create_path(&home_dir, arg_path);

	// let path1 = Path::new(&arg_path);
	// env::set_current_dir(&path1).is_ok();
	// println!("Successfully changed working directory to {}!", path1.display());

	// change_dir(&home_dir, &current_dir);

	// Command::new("ls")
	// 	.current_dir(&path1)
 //        .spawn()
 //        .expect("command failed to start");

 	// let path = current_dir();
 	// println!("{:?}", path);
}


