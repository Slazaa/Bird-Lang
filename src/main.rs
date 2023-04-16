use std::{env, fs};
use clang_format::clang_format;

mod parser;
mod type_infer;
mod comptime;
mod transpiler;

fn main() {
	let args: Vec<String> = env::args()
		.skip(1)
		.collect();

	let file_path = match args.get(0) {
		Some(x) => x,
		None => {
			println!("Expected file path, found nothing");
			return;
		}
	};

	let source = match fs::read_to_string(file_path) {
		Ok(x) => x,
		Err(_) => {
			println!("Could not read file '{file_path}'");
			return;
		}
	};

	let ast = match parser::parse_file(&source) {
		Ok(x) => x,
		Err(e) => {
			println!("{}", e);
			return;
		}
	};

	println!("--- AST ---\n{:#?}", ast);

	let type_infered = match type_infer::infer_file(&ast) {
		Ok(x) => x,
		Err(e) => {
			println!("{}", e);
			return;
		}
	};

	println!("--- Type infered ---\n{:#?}", type_infered);

	let c_source = match clang_format(&transpiler::c::transpile_file(&type_infered)) {
		Ok(x) => x,
		Err(e) => {
			println!("{:?}", e);
			return;
		}
	};

	println!("--- C SOURCE ---\n{}", c_source);
}