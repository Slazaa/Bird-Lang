pub mod compile;
pub mod constants;
pub mod feedback;
pub mod lexer;
pub mod parser;

use std::fs;
use std::path::Path;

use self::compile::c;
use self::feedback::*;
use self::lexer::*;
use self::parser::*;

pub static SRC_FOLDER: &str = "src";

/// The `c` mode compiles the code into C.
pub fn to_c(input_file: &str) -> Result<(), Feedback> {
	filename_to_c(input_file)?;

	//text_to_c(&c::array::array(), PathOrFile::Filename("array.bird".to_owned()))?;

	Ok(())
}

fn filename_to_c(input_file: &str) -> Result<(), Feedback> {
	let text = match fs::read_to_string(input_file) {
		Ok(x) => x,
		Err(_) => return Err(Error::no_file_or_dir(input_file))
	};

	text_to_c(&text, input_file)
}

fn text_to_c(text: &str, input_file: &str) -> Result<(), Feedback> {
	let tokens = Lexer::parse(text, Some(Path::new(input_file)))?;
	let ast = Parser::parse(&tokens)?;

	let output = Path::new(input_file)
		.to_path_buf()
		.set_extension(".c")
		.to_string();

	c::Compiler::compile(&ast, Path::new(input_file), &output)?;

	Ok(())
}
