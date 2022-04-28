pub mod error;
pub mod lexer;

use std::fs;

use self::error::*;

pub fn run(filename: &str) -> Result<(), Error> {
	let file_content = match fs::read_to_string(filename) {
		Ok(x) => x,
		Err(_) => return Err(NoFileOrDirError::new(filename))
	};

	let tokens = match lexer::Lexer::parse(&file_content) {
		Ok(x) => x,
		Err(e) => return Err(e)
	};

	println!("Tokens:");

	for token in tokens.iter() {
		println!("Type: {:?} Symbol: {}", token.token_type(), token.symbol());
	}

	Ok(())
}