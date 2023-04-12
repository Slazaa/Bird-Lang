use crate::parser::exprs::{Expr, r#type::{Type, PtrKind}};

use super::path;

pub fn transpile(input: &Type) -> String {
	let value = match &input.value {
		Expr::Path(value) => path::transpile(value),
		_ => todo!()
	};

	match &input.ptr_kind {
		Some(PtrKind::Const) => format!("const {value}*"),
		Some(PtrKind::Mutable) => format!("{value}*"),
		None => value
	}
}