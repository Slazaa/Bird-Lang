use std::env;

fn main() {
	let args = env::args()
		.skip(1)
		.collect::<Vec<String>>();

	match bird_parser::parse(&args[0]) {
		Ok(x) => println!("{:#?}", x),
		Err(e) => println!("{:?}", e.as_string())
	};
}