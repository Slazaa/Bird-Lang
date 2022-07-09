use std::env;
use std::cmp::Ordering;

use crate::bird::error::Error;

mod bird;

fn main() {
	let args: Vec<String> = env::args().collect();

	match args.len().cmp(&2) {
		Ordering::Less => {
			println!("{}", Error::no_file_or_dir(None, "No input file").as_string());
			return;
		}
		Ordering::Greater => {
			println!("{}", Error::invalid_syntax(None, &format!("Unknown argument '{}'", args[2])).as_string());
			return;
		}
		_ => ()
	}

	match bird::run(&args[1]) {
		Ok(_) => (),
		Err(e) => println!("{}", e.as_string())
	}
}