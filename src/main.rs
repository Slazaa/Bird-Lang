use std::{env, cmp::Ordering};

mod bird;

fn main() {
	let args: Vec<String> = env::args().collect();

	match args.len().cmp(&2) {
		Ordering::Less => {
			println!("No input file");
			return;
		}
		Ordering::Greater => {
			println!("Unknown argument {}", args[2]);
			return;
		}
		_ => ()
	}
/*
	if args.len() < 2 {
		println!("No input file");
		return;
	}

	for arg in args.iter().skip(2) {
		println!("Unknown argument {}", arg);
		return;
	}
*/
	match bird::run(&args[1]) {
		Ok(_) => (),
		Err(e) => println!("{}", e.as_string())
	}
}