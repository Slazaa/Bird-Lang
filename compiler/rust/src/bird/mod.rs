use std::{fs, path::Path};

use self::feedback::{Feedback, Error};

pub mod constants;
pub mod feedback;
pub mod lexer;
pub mod parser;

pub fn compile(filename: &str) -> Result<(), Feedback> {
	let text = match fs::read_to_string(filename) {
		Ok(x) => x,
		Err(_) => return Err(Error::no_file_or_dir(filename))
	};

	let tokens = lexer::Lexer::parse(&text, Some(Path::new(filename)))?;
	let _ast = parser::Parser::parse(&tokens)?;

	todo!();
}