use crate::parser::exprs::literals::int::Int;

pub fn transpile(input: &Int) -> String {
    input.value.to_string()
}