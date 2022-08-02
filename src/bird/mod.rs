pub mod compile;
pub mod constants;
pub mod feedback;
pub mod lexer;
pub mod parser;

use self::feedback::*;
use self::lexer::*;
use self::parser::*;

pub fn run(filename: &str) -> Result<(), Feedback> {
	let tokens = Lexer::parse(filename)?;
	let _ast = Parser::parse(&tokens)?;

	Ok(())
}

pub fn to_c(filename: &str) -> Result<(), Feedback> {
	let tokens = Lexer::parse(filename)?;
	let ast = Parser::parse(&tokens)?;

	compile::c::Compiler::compile(ast)?;

	Ok(())
}