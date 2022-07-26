pub mod constants;
pub mod feedback;
pub mod lexer;
pub mod parser;

use self::feedback::*;

pub fn run(filename: &str) -> Result<(), Feedback> {
	let tokens = lexer::Lexer::parse(filename)?;
	let ast = parser::Parser::parse(&tokens)?;

	println!("{:#?}", ast);

	Ok(())
}

pub fn to_c(filename: &str) -> Result<(), Feedback> {
	let tokens = lexer::Lexer::parse(filename)?;
	let ast = parser::Parser::parse(&tokens)?;

	println!("{:#?}", ast);

	Ok(())
}