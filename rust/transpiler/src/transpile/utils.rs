use bird_parser::patterns::*;
use bird_utils::*;

pub fn rem_ext(filename: &str) -> String {
	filename.split(".").collect::<Vec<&str>>()[0].to_owned()
}

pub fn type_infer(expr: &Expr) -> Result<String, Feedback> {
	Ok(match expr {
		Expr::BinExpr(x) => type_infer(&x.left)?,
		Expr::Literal(x) => match x.kind {
			LiteralKind::Bool => "bool".to_owned(),
			LiteralKind::Chr  => "char_".to_owned(),
			LiteralKind::Flt  => "f32".to_owned(),
			LiteralKind::Int  => "int".to_owned(),
			LiteralKind::Str  => todo!("Strings not supported yet")
		}
		_ => "void".to_owned()
	})
}