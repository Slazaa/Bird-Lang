use std::env;

use crate::bird::feedback::*;

mod bird;

fn main() {
	let args: Vec<String> = env::args().collect();

	match args.len() {
		1 => {
			println!("{}", Error::invalid_syntax(None, "Expecting a mode").as_string());
			return;
		}
		2 => {
			println!("{}", Error::no_input_file().as_string());
			return;
		}
		_ => ()
	}

	let mode = &args[1];
	let filename = &args[2];

	let result = match mode.as_str() {
		"run" => bird::run(filename),
		"c" => bird::to_c(filename),
		_ => {
			println!("{}", Error::invalid_syntax(None, &format!("Invalid mode '{}'", mode)).as_string());
			return;
		}
	};

	if let Err(e) = result {
		println!("{}", e.as_string());
	}
}