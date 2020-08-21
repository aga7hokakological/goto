use std::process::{Command, Stdio};


fn main() {
	Command::new("ls")
        .stdin(Stdio::null())
        .spawn()
        .expect("ls command failed to start");
}
