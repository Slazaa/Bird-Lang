use parse::Token;
use bird_parser::*;
use bird_parser::patterns::*;
use bird_utils::*;

use super::c_header;
use super::utils::*;

pub static UTILS: &str = "\
#ifndef UTILS_H_
#define UTILS_H_

typedef enum {
	false,
	true
} bool;

typedef char i8;
typedef short i16;
typedef long i32;
typedef long long i64;

typedef usigned char u8;
typedef usigned short u16;
typedef usigned long u32;
typedef usigned long long u64;

typedef u32 char_;

#endif\
";

pub struct Transpiler {
	found_main: bool
}

impl Transpiler {
	fn new() -> Self {
		Self {
			found_main: false
		}
	}

	fn found_main(&self) -> bool {
		self.found_main
	}

	fn eval_assign_expr(&mut self, assign_expr: &AssignExpr, scope_depth: usize) -> Result<String, Feedback> {
		Ok(format!("{} = {}", self.eval_expr(&assign_expr.left, scope_depth)?, self.eval_expr(&assign_expr.right, scope_depth)?))
	}

	fn eval_bin_expr(&mut self, bin_expr: &BinExpr, scope_depth: usize) -> Result<String, Feedback> {
		Ok(format!("{} {} {}", self.eval_expr(&bin_expr.left, scope_depth)?, bin_expr.op, self.eval_expr(&bin_expr.right, scope_depth)?))
	}

	fn eval_const_decl(&mut self, const_decl: &ConstDecl, scope_depth: usize) -> Result<String, Feedback> {
		Ok(format!("{} const {} = {}", type_infer(&const_decl.val)?, const_decl.id, self.eval_expr(&const_decl.val, scope_depth)?))
	}

	fn eval_expr(&mut self, expr: &Expr, scope_depth: usize) -> Result<String, Feedback> {
		match expr {
			Expr::AssignExpr(x) => self.eval_assign_expr(x, scope_depth),
			Expr::BinExpr(x) => self.eval_bin_expr(x, scope_depth),
			Expr::ExternBlock(x) => self.eval_extern_block(x, scope_depth),
			Expr::FuncCall(x) => self.eval_func_call(x),
			Expr::IfExpr(x) => self.eval_if_stmt(x, scope_depth),
			Expr::UnaryExpr(x) => self.eval_unary_expr(x, scope_depth),

			Expr::Literal(x) => self.eval_literal(x),
			Expr::Id(x) => self.eval_id(x)
		}
	}

	fn eval_extern_block(&mut self, extern_block: &ExternBlock, scope_depth: usize) -> Result<String, Feedback> {
		if extern_block.lang == "\"C\"" {
			self.eval_stmts(&extern_block.stmts, scope_depth)
		} else {
			Err(Error::unspecified("Extern blocks only support the C language"))
		}
	}

	fn eval_func_call(&mut self, func_call: &FuncCall) -> Result<String, Feedback> {
		Ok(format!("{}()", func_call.id))
	}

	fn eval_func_proto(&mut self, func_proto: &FuncProto) -> Result<String, Feedback> {
		Ok(format!("void {}(void)\n", func_proto.id))
	}

	fn eval_func(&mut self, func: &Func, scope_depth: usize) -> Result<String, Feedback> {
		let id = if func.id == "main" {
			self.found_main = true;
			"main_".to_owned()
		} else {
			func.id.to_owned()
		};

		let mut stmts = self.eval_stmts(&func.stmts, scope_depth + 1)?;

		if stmts.is_empty() {
			stmts = "\n".to_owned();
		}

		if func.public {
			Ok(format!("\n\
void {}(void) {{
{}\
}}\n\
			", id, stmts))
		} else {
			Ok(format!("\n\
static void {}(void) {{
{}\
}}\n\
			", id, stmts))
		}
	}

	fn eval_id(&mut self, id: &Token) -> Result<String, Feedback> {
		Ok(format!("{}", id.symbol))
	}
/*
	fn eval_if_expr(&mut self, if_stmt: &IfExpr, scope_depth: usize) -> Result<String, Feedback> {
		todo!();
	}
*/
	fn eval_if_stmt(&mut self, if_stmt: &IfExpr, scope_depth: usize) -> Result<String, Feedback> {
		let scope_tabs = "\t".repeat(scope_depth);

		let mut stmts = self.eval_stmts(&if_stmt.stmts, scope_depth + 1)?;

		if stmts.is_empty() {
			stmts = "\n".to_owned();
		}

		Ok(format!("\n\
{scope_tabs}if ({}) {{
{}\
{scope_tabs}}}\n\
		", self.eval_expr(&if_stmt.cond, scope_depth)?, stmts))
	}

	fn eval_item(&mut self, item: &Item, scope_depth: usize) -> Result<String, Feedback> {
		match item {
			Item::ConstDecl(x) => self.eval_const_decl(x, scope_depth),
			Item::Func(x) => self.eval_func(x, scope_depth),
			Item::FuncProto(x) => self.eval_func_proto(x),
			Item::VarDecl(x) => self.eval_var_decl(x, scope_depth)
		}
	}
	
	fn eval_literal(&mut self, literal: &Literal) -> Result<String, Feedback> {
		Ok(format!("{}", literal.value))
	}

	fn eval_program(&mut self, program: &Program) -> Result<String, Feedback> {
		self.eval_stmts(&program.stmts, 0)
	}

	fn eval_stmt(&mut self, stmt: &Stmt, scope_depth: usize) -> Result<String, Feedback> {
		let scope_tabs = "\t".repeat(scope_depth);

		Ok(match stmt {
			Stmt::Expr(x) => {
				let expr = self.eval_expr(x, scope_depth)?;

				let expr = match x {
					Expr::IfExpr(_) => expr,
					_ => scope_tabs + &expr + ";\n"
				};

				expr
			}
			Stmt::Item(x) => {
				let item = self.eval_item(x, scope_depth)?;

				let item = match x {
					Item::Func(_) => item,
					_ => scope_tabs + &item + ";\n"
				};

				item
			}
		})
	}

	fn eval_stmts(&mut self, stmts: &Stmts, scope_depth: usize) -> Result<String, Feedback> {
		let mut res = String::new();

		for stmt in &stmts.stmts {
			res.push_str(&self.eval_stmt(stmt, scope_depth)?)
		}

		Ok(res)
	}

	fn eval_type(&mut self, type_: &Type) -> Result<String, Feedback> {
		match type_.ptr_kind {
			PtrKind::None => Ok(format!("{}", type_.id)),
			PtrKind::Const => Ok(format!("const {}*", type_.id)),
			PtrKind::Mut => Ok(format!("{}*", type_.id))
		}
	}

	fn eval_unary_expr(&mut self, unary_expr: &UnaryExpr, scope_depth: usize) -> Result<String, Feedback> {
		Ok(format!("{}{}", unary_expr.op, self.eval_expr(&unary_expr.val, scope_depth)?))
	}

	fn eval_var_decl(&mut self, var_decl: &VarDecl, scope_depth: usize) -> Result<String, Feedback> {
		Ok(match &var_decl.val {
			Some(val) => format!("{} {} = {}", type_infer(val)?, var_decl.id, self.eval_expr(val, scope_depth)?),
			None => format!("void {}", var_decl.id)
		})
	}
}

pub fn transpile(ast: &Node) -> Result<(String, String), Feedback> {
	let mut transpiler = Transpiler::new();

	if let Node::Program(program) = ast {
		let mut src = transpiler.eval_program(program)?;

		if transpiler.found_main() {
			src.push_str("
int main(int argc, char** argv) {
	main_();

	return 0;
}
			");
		} else {
			let filename = rem_ext(program.loc.filename.as_ref().unwrap());

			src = format!("\
#include \"{}.h\"

{}\
			", filename, src);
		}

		Ok((src, c_header::transpile(ast)?))
	} else {
		Err(Error::unspecified("Expected Program"))
	}
}