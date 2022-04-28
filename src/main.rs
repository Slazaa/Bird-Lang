use std::env;

mod bird;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		println!("No input file");
		return;
	}

	for arg in args.iter().skip(2) {
		println!("Unknown argument {}", arg);
		return;
	}

	match bird::run(&args[1]) {
		Ok(_) => (),
		Err(e) => println!("{}", e.as_string())
	}
}