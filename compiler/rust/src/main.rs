use std::env;

use crate::bird::feedback::*;

mod bird;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();

	if args.is_empty() {
		println!("{}", Error::invalid_syntax(None, "Expecting an input file").as_string());
		return;
	}

	if args.len() > 1 {
		println!("{}",Error::invalid_syntax(None, "Too much arguments were given").as_string());
		return;
	}

	let filename = &args[1];

	if let Err(feedback) = bird::compile(filename) {
		println!("{}", feedback.as_string());
	}
}
