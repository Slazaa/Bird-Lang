use crate::parser::exprs::literals::char::Char;

pub fn transpile(input: &Char) -> String {
	format!("0x{:X}", match input.value {
		"\\n" => 0xA,
		"\\t" => 0x9,
		x => x.chars().nth(0).unwrap() as u32
	})
}