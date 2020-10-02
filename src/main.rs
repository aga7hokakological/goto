use std::env;
#[macro_use] extern crate lazy_static;

pub mod dir;

use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(test)]
fn func() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num  += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


fn main() {
	let args: Vec<String> = env::args().collect();
	
	let arg_path = &args[1];

	let home_dir = dirs::home_dir().unwrap();

	let _current_dir = env::current_dir().unwrap();

	// let mut p = PathBuf::from("");
	// p.push(&path::MAIN_SEPARATOR.to_string());
	// p.push(&arg_path);
	// println!("{}", p.to_str().unwrap());

	dir::recursive_file_traversal(&home_dir, arg_path);
}
