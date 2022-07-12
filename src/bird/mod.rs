pub mod feedback;
pub mod lexer;
pub mod parser;

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