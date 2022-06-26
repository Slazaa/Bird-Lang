pub mod error;
pub mod lexer;
pub mod parser;

use self::error::*;

pub fn run(filename: &str) -> Result<(), Error> {
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