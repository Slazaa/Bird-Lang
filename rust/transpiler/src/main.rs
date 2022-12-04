use std::env;
use std::fs;

use birdt::transpile;

fn main() {
	let args = env::args()
		.skip(1)
		.collect::<Vec<String>>();

	let filename = &args[0];

	let ast = match bird_parser::parse(filename) {
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

	fs::write("__utils__.h", transpile::c::UTILS).unwrap();
	fs::write(transpile::utils::rem_ext(filename) + ".c", source.as_bytes()).unwrap();
	fs::write(transpile::utils::rem_ext(filename) + ".h", header.as_bytes()).unwrap();
}