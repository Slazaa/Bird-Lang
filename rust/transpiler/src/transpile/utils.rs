use bird_parser::patterns::*;
use bird_utils::*;

pub fn rem_ext(filename: &str) -> String {
	filename.split(".").collect::<Vec<&str>>()[0].to_owned()
}

pub fn type_infer(expr: &Expr) -> Result<String, Feedback> {
	Ok(match expr {
		Expr::BinExpr(x) => type_infer(&x.left)?,
		Expr::Literal(x) => {
			let value = &x.value;

			match x.kind {
				LiteralKind::Bool => "bool".to_owned(),
				LiteralKind::Chr  => "__char__".to_owned(),
				LiteralKind::Flt  => {
					if 	value.parse::<f64>().unwrap() >= f32::MAX.into() {
						"f64".to_owned()
					} else {
						"f32".to_owned()
					}
				}
				LiteralKind::Int  => {
					if value.parse::<i64>().unwrap() >= i32::MAX.into() {
						"i64".to_owned()
					} else {
						"i32".to_owned()
					}
				}
				LiteralKind::Str  => todo!("Strings not supported yet")
			}
		}
		_ => "void".to_owned()
	})
}