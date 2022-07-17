pub mod constants;
pub mod feedback;
pub mod lexer;
pub mod parser;
pub mod pattern_finder;
pub mod translator;

use std::io::Write;
use std::fs::File;

use self::feedback::*;

pub fn run(filename: &str) -> Result<(), Feedback> {
	let tokens = match lexer::Lexer::parse(filename) {
		Ok(x) => x,
		Err(e) => return Err(e)
	};

	let ast = match parser::Parser::parse(tokens) {
		Ok(x) => x,
		Err(e) => return Err(e)
	};

	println!("{:#?}", ast);

	Ok(())
}

pub fn to_c(filename: &str) -> Result<(), Feedback> {
	let tokens = match lexer::Lexer::parse(filename) {
		Ok(x) => x,
		Err(e) => return Err(e)
	};

	let result = match translator::c::Translator::translate(filename, tokens) {
		Ok(x) => x,
		Err(e) => return Err(e)
	};

	let mut file = File::create("main.c").unwrap();

	write!(&mut file, "{}", result).unwrap();

	Ok(())
}