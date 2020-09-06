use std::{env, fs};
use std::path::Path;
// use std::process;

// #[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

pub fn change_dir(new_dir: String) {
	let path1 = Path::new(&new_dir);
	env::set_current_dir(&path1).unwrap();
	println!("Successfully changed working directory to {}!", path1.display());
}


pub fn recursive_file_traversal<P>(home_dir: P, arg_path: &std::string::String) -> String
where
    P: AsRef<Path> {

    let new_directory = String::from("");

    // One time compilation
    lazy_static! {
    	static ref RE: Regex = Regex::new(r"(\w+)$").unwrap();
    }

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
                    let match_path = RE.find(&new_directory).unwrap();
                    // println!("{:?}", match_path);

                    let start = match_path.start();
                    let end = match_path.end();

                    if &new_directory[start..end] == arg_path.as_str() {
                    	println!("{:?}", new_directory);
                    	change_dir(new_directory);
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
    new_directory.to_string()
}
