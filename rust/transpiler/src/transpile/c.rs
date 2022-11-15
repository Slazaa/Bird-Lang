use bird_parser::*;
use bird_parser::patterns::*;
use bird_utils::feedback::{Feedback, Error};

fn type_infer(expr: &Expr) -> Result<String, Feedback> {
	Ok(match expr {
		Expr::BinExpr(x) => type_infer(&x.left)?,
		Expr::Literal(x) => match x.kind {
			LiteralKind::Bool => "bool".to_owned(),
			LiteralKind::Chr  => "char".to_owned(),
			LiteralKind::Flt  => "f32".to_owned(),
			LiteralKind::Int  => "i32".to_owned(),
			LiteralKind::Str  => "char*".to_owned()
		},

		_ => "void".to_owned()
	})
}

fn eval_bin_expr(bin_expr: &BinExpr, scope_depth: usize) -> Result<String, Feedback> {
	Ok(format!("{} {} {}", eval_expr(&bin_expr.left, scope_depth)?, bin_expr.op, eval_expr(&bin_expr.right, scope_depth)?))
}

fn eval_expr(expr: &Expr, scope_depth: usize) -> Result<String, Feedback> {
	match expr {
		Expr::BinExpr(x) => eval_bin_expr(x, scope_depth),
		Expr::IfExpr(x) => eval_if_stmt(x, scope_depth),
		Expr::UnaryExpr(x) => eval_unary_expr(x, scope_depth),

		Expr::Literal(x) => eval_literal(x),
		Expr::Id(x) => eval_id(x),

		_ => todo!("{:#?}", expr)
	}
}

fn eval_func(func: &Func, scope_depth: usize) -> Result<String, Feedback> {
	let scope_tabs = "\t".repeat(scope_depth);

	Ok(format!("\
{scope_tabs}void {}(void) {{
{}\
{scope_tabs}}}\
	", func.id, eval_stmts(&func.stmts, scope_depth + 1)?))
}

fn eval_func_proto(func_proto: &FuncProto) -> Result<String, Feedback> {
	todo!();
}

fn eval_id(id: &String) -> Result<String, Feedback> {
	Ok(format!("{}", id))
}

fn eval_if_expr(if_stmt: &IfExpr, scope_depth: usize) -> Result<String, Feedback> {
	todo!();
}

fn eval_if_stmt(if_stmt: &IfExpr, scope_depth: usize) -> Result<String, Feedback> {
	let scope_tabs = "\t".repeat(scope_depth);

	Ok(format!("\n\
{scope_tabs}if ({}) {{
{}\
{scope_tabs}}}\n\
	", eval_expr(&if_stmt.cond, scope_depth)?, eval_stmts(&if_stmt.stmts, scope_depth + 1)?))
}

fn eval_item(item: &Item, scope_depth: usize) -> Result<String, Feedback> {
	match item {
		Item::Func(x) => eval_func(x, scope_depth),
		Item::FuncProto(x) => eval_func_proto(x),
		Item::VarDecl(x) => eval_var_decl(x, scope_depth)
	}
}

fn eval_literal(literal: &Literal) -> Result<String, Feedback> {
	Ok(format!("{}", literal.value))
}

fn eval_program(program: &Program) -> Result<String, Feedback> {
	if let Some(stmts) = &program.stmts {
		eval_stmts(stmts, 0)
	} else {
		Ok("".to_owned())
	}
}

fn eval_stmt(stmt: &Stmt, scope_depth: usize) -> Result<String, Feedback> {
	match stmt {
		Stmt::Expr(x) => eval_expr(x, scope_depth),
		Stmt::Item(x) => eval_item(x, scope_depth)
	}
}

fn eval_stmts(stmts: &Stmts, scope_depth: usize) -> Result<String, Feedback> {
	let mut res = String::new();

	for stmt in &stmts.stmts {
		match eval_stmt(stmt, scope_depth) {
			Ok(x) => res.push_str(&x),
			Err(e) => return Err(e)
		}
	}

	Ok(res)
}

fn eval_unary_expr(unary_expr: &UnaryExpr, scope_depth: usize) -> Result<String, Feedback> {
	Ok(format!("{}{}", unary_expr.op, eval_expr(&unary_expr.val, scope_depth)?))
}

fn eval_var_decl(var_decl: &VarDecl, scope_depth: usize) -> Result<String, Feedback> {
	let scope_tabs = "\t".repeat(scope_depth);

	if let Some(val) = &var_decl.val {
		Ok(match val {
			Expr::IfExpr(x) => format!("\
{scope_tabs}void {};
{}\
			", var_decl.id, eval_if_expr(x, scope_depth)?),
			_ => format!("{scope_tabs}{} {} = {};\n", type_infer(val)?, var_decl.id, eval_expr(val, scope_depth)?)
		})
	} else {
		Ok(format!("{scope_tabs}void {};\n", var_decl.id))
	}
}

pub fn transpile(ast: &Node) -> Result<String, Feedback> {
	if let Node::Program(program) = ast {
		eval_program(program)
	} else {
		Err(Error::unspecified("Expected Program"))
	}
}