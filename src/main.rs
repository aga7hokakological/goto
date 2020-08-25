use std::env;
use std::process::{Command, Stdio};


fn main() {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);

	let path = &args[1];

	Command::new("cd")
		.current_dir(path)
        .spawn()
        .expect("command failed to start");
}
