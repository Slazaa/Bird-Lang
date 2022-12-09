use parse::Token;
use bird_parser::*;
use bird_parser::patterns::*;
use bird_utils::*;

use super::c_header;
use super::utils::*;

pub static UTILS: &str = "\
#ifndef __UTILS_H__
#define __UTILS_H__

typedef enum {
	false,
	true
} bool;

typedef char i8;
typedef short i16;
typedef long i32;
typedef long long i64;

typedef unsigned int uint;

typedef unsigned char u8;
typedef unsigned short u16;
typedef unsigned long u32;
typedef unsigned long long u64;

typedef float f32;
typedef double f64;

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
		let static_str = match const_decl.public {
			Some(public) => if !public {
				"static "
			} else {
				""
			},
			None => ""
		};

		Ok(format!("{}{} const {} = {}", static_str, type_infer(&const_decl.val)?, const_decl.id, self.eval_expr(&const_decl.val, scope_depth)?))
	}

	fn eval_expr(&mut self, expr: &Expr, scope_depth: usize) -> Result<String, Feedback> {
		match expr {
			Expr::AssignExpr(x) => self.eval_assign_expr(x, scope_depth),
			Expr::BinExpr(x) => self.eval_bin_expr(x, scope_depth),
			Expr::FuncCall(x) => self.eval_func_call(x),
			Expr::IfExpr(x) => self.eval_if_stmt(x, scope_depth),
			Expr::UnaryExpr(x) => self.eval_unary_expr(x, scope_depth),

			Expr::Literal(x) => self.eval_literal(x),
			Expr::Id(x) => self.eval_id(x)
		}
	}

	fn eval_field(&mut self, field: &Field) -> Result<String, Feedback> {
		Ok(format!("\t{} {};\n", self.eval_type(&field.type_)?, field.id))
	}

	fn eval_fields(&mut self, fields: &Fields) -> Result<String, Feedback> {
		let mut res = String::new();

		for field in &fields.fields {
			res.push_str(&self.eval_field(field)?)
		}

		Ok(res)
	}

	fn eval_func_call(&mut self, func_call: &FuncCall) -> Result<String, Feedback> {
		Ok(format!("{}()", func_call.id))
	}

	fn eval_func_proto(&mut self, func_proto: &FuncProto) -> Result<String, Feedback> {
		Ok(format!("void {}(void)", func_proto.id))
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

		let static_str = if !func.public.unwrap() {
			"static "
		} else {
			""
		};

		Ok(format!("\n\
{}void {}(void) {{
{}\
}}\n\
			", static_str, id, stmts)
		)
	}

	fn eval_id(&mut self, id: &Token) -> Result<String, Feedback> {
		Ok(format!("{}", id.symbol))
	}
	
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

	fn eval_import(&mut self, import: &Import) -> Result<String, Feedback> {
		if import.public.unwrap() {
			Ok("".to_owned())
		} else {
			let path = rem_ext(&import.path) + ".h";
			Ok(format!("#include {}\"\n", path))
		}
	}

	fn eval_item(&mut self, item: &Item, scope_depth: usize) -> Result<String, Feedback> {
		match item {
			Item::ConstDecl(x) => self.eval_const_decl(x, scope_depth),
			Item::Func(x) => self.eval_func(x, scope_depth),
			Item::FuncProto(x) => self.eval_func_proto(x),
			Item::Import(x) => self.eval_import(x),
			Item::Struct(x) => self.eval_struct(x),
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

		match stmt {
			Stmt::Expr(x) => {
				let expr = self.eval_expr(x, scope_depth)?;

				let expr = match x {
					Expr::IfExpr(_) => expr,
					_ => scope_tabs + &expr + ";\n"
				};

				Ok(expr)
			}
			Stmt::Item(x) => {
				let item = self.eval_item(x, scope_depth)?;

				let item = match x {
					Item::Func(_)   |
					Item::Import(_) |
					Item::Struct(_) => item,
					_ => scope_tabs + &item + ";\n"
				};

				Ok(item)
			}
		}
	}

	fn eval_stmts(&mut self, stmts: &Stmts, scope_depth: usize) -> Result<String, Feedback> {
		let mut res = String::new();

		for stmt in &stmts.stmts {
			res.push_str(&self.eval_stmt(stmt, scope_depth)?)
		}

		Ok(res)
	}

	fn eval_struct(&mut self, struct_: &Struct) -> Result<String, Feedback> {
		if struct_.public.unwrap() {
			Ok(format!("\
typedef struct {{
{}\
}} {};\n\
			", self.eval_fields(&struct_.fields)?, struct_.id))
		} else {
			Ok("".to_owned())
		}
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
		let static_str = match var_decl.public {
			Some(public) => if !public {
				"static "
			} else {
				""
			},
			None => ""
		};

		Ok(match &var_decl.val {
			Some(val) => format!("{}{} {} = {}", static_str, type_infer(val)?, var_decl.id, self.eval_expr(val, scope_depth)?),
			None => format!("{}void {}", static_str, var_decl.id)
		})
	}
}

pub fn transpile(ast: &Node) -> Result<(String, String), Feedback> {
	let mut transpiler = Transpiler::new();

	if let Node::Program(program) = ast {
		let mut src = transpiler.eval_program(program)?;
		let filename = rem_ext(program.loc.filename.as_ref().unwrap());

		if transpiler.found_main() {
			src = format!("\
#include \"{}.h\"
#include \"__utils__.h\"

{}

int main(int argc, char* argv[]) {{
	main_();

	return 0;
}}\
			", filename, src);
		} else {
			src = format!("\
#include \"{}.h\"
#include \"__utils__.h\"

{}\
			", filename, src);
		}

		Ok((src, c_header::transpile(ast)?))
	} else {
		Err(Error::unspecified("Expected Program"))
	}
}