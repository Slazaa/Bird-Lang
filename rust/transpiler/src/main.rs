use std::env;

mod transpile;

fn main() {
	let args = env::args()
		.skip(1)
		.collect::<Vec<String>>();

	let ast = match bird_parser::parse(&args[0]) {
		Ok(x) => x,
		Err(e) => {
			println!("{}", e);
			return;
		}
	};

	let (source, header) = match transpile::c::transpile(&ast) {
		Ok(x) => x,
		Err(e) => {
			println!("{}", e);
			return;
		}
	};

	println!("----- SOURCE -----");
	println!("{}", source);

	println!("\n----- HEADER -----");
	println!("{}", header);
}