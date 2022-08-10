pub mod compile;
pub mod constants;
pub mod feedback;
pub mod lexer;
pub mod parser;

use self::feedback::*;
use self::lexer::*;
use self::parser::*;

/// The `run` mode automatically runs the program.
pub fn run(filename: &str) -> Result<(), Feedback> {
	let tokens = Lexer::parse(filename)?;
	let _ast = Parser::parse(&tokens)?;

	Ok(())
}

/// The `c` mode compiles the code into C.
pub fn to_c(filename: &str) -> Result<(), Feedback> {
	use self::compile::c;

	let tokens = Lexer::parse(filename)?;
	let ast = Parser::parse(&tokens)?;

	println!("{:#?}", ast);

	c::types::types_file()?;
	//c::array::array()?;

	compile::c::Compiler::compile(ast)?;

	Ok(())
}