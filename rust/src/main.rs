use std::env;

use crate::bird::feedback::*;

mod bird;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.len() {
        0 => {
            println!(
                "{}",
                Error::invalid_syntax(None, "Expecting a mode").as_string()
            );
            return;
        }
        1 => {
            println!(
                "{}",
                Error::invalid_syntax(None, "Expecting an output file").as_string()
            );
            return;
        }
        _ => (),
    }

    if args.len() > 1 {
        println!(
            "{}",
            Error::invalid_syntax(None, "Too much arguments were given").as_string()
        );
        return;
    }

    let mode = &args[0];
    let output = &args[1];

    let result = match mode.as_str() {
        "c" => bird::to_c(output),
        _ => {
            println!(
                "{}",
                Error::invalid_syntax(None, &format!("Invalid mode '{}'", mode)).as_string()
            );
            return;
        }
    };

    if let Err(e) = result {
        println!("{}", e.as_string());
    }
}
