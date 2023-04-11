use crate::parser::exprs::literals::bool::Bool;

pub fn transpile(input: &Bool) -> String {
    format!("{}", input.value)
}