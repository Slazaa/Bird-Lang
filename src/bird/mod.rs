pub mod constants;
pub mod feedback;
pub mod lexer;
pub mod parser;
pub mod pattern_finder;
pub mod translator;

use std::io::Write;
use std::fs::{File, self};

use self::feedback::*;

pub fn run(filename: &str) -> Result<(), Feedback> {
	let tokens = match lexer::Lexer::parse(filename) {
		Ok(x) => x,
		Err(e) => return Err(e)
	};

	let ast = match parser::Parser::parse(&tokens) {
		Ok(x) => x,
		Err(e) => return Err(e)
	};

	println!("{:#?}", ast);

	Ok(())
}

pub fn to_c(filename: &str) -> Result<(), Feedback> {
	use translator::c::*;

	let tokens = match lexer::Lexer::parse(filename) {
		Ok(x) => x,
		Err(e) => return Err(e)
	};

	match parser::Parser::parse(&tokens) {
		Ok(_) => (),
		Err(e) => return Err(e)
	};

	match fs::create_dir_all("c/bird") {
		Ok(_) => (),
		Err(_) => return Err(Error::unspecified("Failed creating 'c' directory"))
	}

	match File::create("c/bird/types.h") {
		Ok(mut file) => match write!(file, "{}", bird::types()) {
			Ok(_) => (),
			Err(_) => return Err(Error::unspecified("Failed writing to file"))
		}
		Err(_) => return Err(Error::unspecified("Failed creating 'c/bird/types.h' file"))
	}

	let mut main_c = match File::create("c/main.c") {
		Ok(x) => x,
		Err(_) => return Err(Error::unspecified("Failed creating file"))
	};

	let mut result = String::new();

	result.push_str("\
#include \"bird/types.h\"\n\n\
	");

	match translate::Translator::translate(filename, tokens) {
		Ok(x) => result.push_str(&x),
		Err(e) => return Err(e)
	};

	match write!(&mut main_c, "{}", result) {
		Ok(_) => (),
		Err(_) => return Err(Error::unspecified("Failed writing to file"))
	}

	Ok(())
}