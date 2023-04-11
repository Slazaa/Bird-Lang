use crate::parser::exprs::assign::Assign;

pub fn transpile(input: &Assign) -> String {
	format!("{} = {}", super::transpile(&input.expr), super::transpile(&input.value))
}