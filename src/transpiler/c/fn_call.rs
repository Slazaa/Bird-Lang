use crate::parser::exprs::{fn_call::FnCall, Expr};

pub fn transpile_inputs(input: &[Expr]) -> String {
	let mut res = if input.is_empty() {
		String::new()
	} else {
		super::transpile(&input[0])
	};

	for input in input.iter().skip(1) {
		res += &format!(", {}", super::transpile(input));
	}

	res
}

pub fn transpile(input: &FnCall) -> String {
	let expr = super::transpile(&input.expr);
	let param_decls = transpile_inputs(&input.inputs);

	format!("{expr}({param_decls})")
}