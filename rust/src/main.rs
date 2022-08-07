use std::env;

use crate::bird::feedback::*;

mod bird;

fn main() {
	let args: Vec<String> = env::args().skip(1)
		.collect();

	match args.len() {
		0 => {
			println!("{}", Error::invalid_syntax(None, "Expecting a mode").as_string());
			return;
		}
		1 => {
			println!("{}", Error::no_input_file().as_string());
			return;
		}
		_ => ()
	}

	let mode = &args[0];
	let filename = &args[1];

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