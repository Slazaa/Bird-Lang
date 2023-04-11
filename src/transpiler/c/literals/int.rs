use crate::parser::exprs::literals::int::Int;

pub fn transpile(input: &Int) -> String {
    format!("{}", input.value)
}