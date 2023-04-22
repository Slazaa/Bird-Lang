use crate::parser::exprs::{Expr, fn_decl::{ParamDecl, FnDecl}, vis::Vis, literals::int::Int, r#return::Return};
use super::{ident, r#type, block};

pub fn transpile_param_decl(input: &ParamDecl) -> String {
	let ident = ident::transpile(&input.ident);
	let r#type = r#type::transpile(&input.r#type);

	format!("{type} {ident}")
}

pub fn transpile_param_decls(input: &[ParamDecl]) -> String {
	let mut res = if input.is_empty() {
		return "void".to_owned();
	} else {
		transpile_param_decl(&input[0])
	};

	for input in input.iter().skip(1) {
		res += &format!(", {}", transpile_param_decl(input));
	}

	res
}

pub fn transpile_sig(input: &FnDecl) -> String {
	let vis = match input.vis {
		Vis::Private => "static ".to_string(),
		Vis::Public => "".to_string()
	};

	let ident = ident::transpile(&input.ident);
	let inputs = transpile_param_decls(&input.inputs);

	let output = match &input.output {
		Some(output) => r#type::transpile(output),
		None => "void".to_owned()
	};

	format!("{vis}{output} {ident}({inputs});")
}

pub fn transpile(input: &FnDecl) -> String {
	let ident = ident::transpile(&input.ident);
	let inputs = transpile_param_decls(&input.inputs);

	let output = if ident != "main" {
		match &input.output {
			Some(output) => r#type::transpile(output),
			None => "void".to_owned()
		}
	} else {
		"int".to_owned()
	};

	let mut body = input.body.clone();
	
	if ident == "main" {
		body.exprs.push(Expr::Return(Box::new(Return { value: Expr::Int(Int { value: "0" }) })))
	}

	let body = block::transpile(&body);

	format!("{output} {ident}({inputs}) {body}")
}
