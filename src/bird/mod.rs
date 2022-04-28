pub mod error;
pub mod lexer;

use self::error::*;

pub fn run(filename: &str) -> Result<(), Error> {
	let tokens = match lexer::Lexer::parse(&filename) {
		Ok(x) => x,
		Err(e) => return Err(e)
	};

	println!("Tokens:");

	for token in tokens.iter() {
		println!("Type: {:?} Symbol: {}", token.token_type(), token.symbol());
	}

	Ok(())
}