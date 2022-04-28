use std::env;

mod bird;

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

	match bird::run(&args[1]) {
		Ok(_) => (),
		Err(e) => print!("[ERROR] {}", e.as_string())
	}
}