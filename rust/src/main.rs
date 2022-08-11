use std::env;

use crate::bird::feedback::*;

mod bird;

fn main() {
	let args: Vec<String> = env::args().skip(1)
		.collect();

	if args.is_empty() {
		println!("{}", Error::invalid_syntax(None, "Expecting a mode").as_string());
		return;
	}
	
	if args.len() > 1 {
		println!("{}", Error::invalid_syntax(None, "Too much arguments were given").as_string());
		return;
	}

	let mode = &args[0];

	let result = match mode.as_str() {
		"c" => bird::to_c(),
		_ => {
			println!("{}", Error::invalid_syntax(None, &format!("Invalid mode '{}'", mode)).as_string());
			return;
		}
	};

	if let Err(e) = result {
		println!("{}", e.as_string());
	}
}