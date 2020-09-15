use std::{
    fs,
    path::Path,
    process,
};

extern crate exitcode;
extern crate regex;
use regex::Regex;

#[test]
fn dir() {

    let new_dir = String::from("~/Desktop");

    let path1 = Path::new(&new_dir);
    env::set_current_dir(&path1).unwrap();
    println!("{}!", path1.display());
}

pub fn change_dir(new_dir: String) -> String {
    // println!("{}", new_dir);

    match std::env::set_current_dir(&new_dir) {
        Ok(_) => process::exit(exitcode::OK),  
        Err(e) => {
            println!("{}", e);
        }
    }

    return new_dir
}


pub fn recursive_file_traversal<P>(home_dir: P, arg_path: &std::string::String) -> String
where
    P: AsRef<Path> {

    let new_directory = String::from("");

    // One time compilation
    lazy_static! {
    	static ref RE: Regex = Regex::new(r"(\.\w+|\w+)$").unwrap();
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

                    let match_path = RE.captures(&new_directory).unwrap();

                    if &match_path[0] == arg_path.as_str() {
                        change_dir(new_directory);
                    	break;
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
