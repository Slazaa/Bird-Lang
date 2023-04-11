use crate::parser::exprs::literals::string::String as StringExpr;

pub fn transpile(input: &StringExpr) -> String {
	format!("\"{}\"", input.value)
}