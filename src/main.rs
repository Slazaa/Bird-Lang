use std::env;

mod scanner;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		println!("[ERROR] No input file");
		return;
	}

	for arg in args.iter().skip(2) {
		println!("[ERROR] Unknown argument {}", arg);
		return;
	}

	let tokens = match scanner::tokenize_file(args[1].as_str()) {
		Ok(x) => x,
		Err(e) => {
			println!("[ERROR] {}", e);
			return;
		}
	};

	scanner::print_tokens(&tokens);
}