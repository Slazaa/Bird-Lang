use crate::parser::exprs::literals::float::Float;

pub fn transpile(input: &Float) -> String {
	input.value.to_string()	
}