use std::env;

mod transpile;

fn main() {
	let args = env::args()
		.skip(1)
		.collect::<Vec<String>>();

	let ast = match bird_parser::parse(&args[0]) {
		Ok(x) => x,
		Err(e) => {
			println!("{}", e.as_string());
			return;
		}
	};

	let transpiled_code = match transpile::c::transpile(&ast) {
		Ok(x) => x,
		Err(e) => {
			println!("{}", e.as_string());
			return;
		}
	};

	println!("{}", transpiled_code);
}