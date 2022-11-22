use bird_parser::*;
use bird_parser::patterns::*;
use bird_utils::feedback::{Feedback, Error};

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

	fn type_infer(&mut self, expr: &Expr) -> Result<String, Feedback> {
		Ok(match expr {
			Expr::BinExpr(x) => self.type_infer(&x.left)?,
			Expr::Literal(x) => match x.kind {
				LiteralKind::Bool => "bool".to_owned(),
				LiteralKind::Chr  => "char".to_owned(),
				LiteralKind::Flt  => "f32".to_owned(),
				LiteralKind::Int  => "int".to_owned(),
				LiteralKind::Str  => todo!("Strings not supported yet")
			},

			_ => "void".to_owned()
		})
	}

	fn eval_bin_expr(&mut self, bin_expr: &BinExpr, scope_depth: usize) -> Result<String, Feedback> {
		Ok(format!("{} {} {}", self.eval_expr(&bin_expr.left, scope_depth)?, bin_expr.op, self.eval_expr(&bin_expr.right, scope_depth)?))
	}

	fn eval_expr(&mut self, expr: &Expr, scope_depth: usize) -> Result<String, Feedback> {
		match expr {
			Expr::BinExpr(x) => self.eval_bin_expr(x, scope_depth),
			Expr::IfExpr(x) => self.eval_if_stmt(x, scope_depth),
			Expr::UnaryExpr(x) => self.eval_unary_expr(x, scope_depth),

			Expr::Literal(x) => self.eval_literal(x),
			Expr::Id(x) => self.eval_id(x),

			_ => todo!("{:#?}", expr)
		}
	}

	fn eval_extern_block(&mut self, extern_block: &ExternBlock, scope_depth: usize) -> Result<String, Feedback> {
		if extern_block.lang != "\"C\"" {
			return Err(Error::unspecified("Extern blocks only support the C language"));
		}

		self.eval_items(&extern_block.items, scope_depth)
	}

	fn eval_func(&mut self, func: &Func, scope_depth: usize) -> Result<String, Feedback> {
		let scope_tabs = "\t".repeat(scope_depth);

		let id = if func.id == "main" {
			self.found_main = true;
			"main_".to_owned()
		} else {
			(&func.id).to_owned()
		};

		Ok(format!("\n\
{scope_tabs}void {}(void) {{
{}\
{scope_tabs}}}\
		", id, self.eval_stmts(&func.stmts, scope_depth + 1)?))
	}

	fn eval_func_proto(&mut self, func_proto: &FuncProto) -> Result<String, Feedback> {
		Ok(format!("void {}(void)\n", func_proto.id))
	}

	fn eval_id(&mut self, id: &String) -> Result<String, Feedback> {
		Ok(format!("{}", id))
	}

	fn eval_if_expr(&mut self, if_stmt: &IfExpr, scope_depth: usize) -> Result<String, Feedback> {
		todo!();
	}

	fn eval_if_stmt(&mut self, if_stmt: &IfExpr, scope_depth: usize) -> Result<String, Feedback> {
		let scope_tabs = "\t".repeat(scope_depth);

		Ok(format!("\n\
{scope_tabs}if ({}) {{
{}\
{scope_tabs}}}\n\
		", self.eval_expr(&if_stmt.cond, scope_depth)?, self.eval_stmts(&if_stmt.stmts, scope_depth + 1)?))
	}

	fn eval_item(&mut self, item: &Item, scope_depth: usize) -> Result<String, Feedback> {
		match item {
			Item::ExternBlock(x) => self.eval_extern_block(x, scope_depth),
			Item::Func(x) => self.eval_func(x, scope_depth),
			Item::FuncProto(x) => self.eval_func_proto(x),
			Item::VarDecl(x) => self.eval_var_decl(x, scope_depth)
		}
	}

	fn eval_items(&mut self, items: &Items, scope_depth: usize) -> Result<String, Feedback> {
		let mut res = String::new();

		for item in &items.items {
			match self.eval_item(item, scope_depth) {
				Ok(x) => res.push_str(&x),
				Err(e) =>  return Err(e)
			}
		}

		Ok(res)
	}

	fn eval_literal(&mut self, literal: &Literal) -> Result<String, Feedback> {
		Ok(format!("{}", literal.value))
	}

	fn eval_program(&mut self, program: &Program) -> Result<String, Feedback> {
		if let Some(stmts) = &program.stmts {
			self.eval_stmts(stmts, 0)
		} else {
			Ok("".to_owned())
		}
	}

	fn eval_stmt(&mut self, stmt: &Stmt, scope_depth: usize) -> Result<String, Feedback> {
		match stmt {
			Stmt::Expr(x) => self.eval_expr(x, scope_depth),
			Stmt::Item(x) => self.eval_item(x, scope_depth)
		}
	}

	fn eval_stmts(&mut self, stmts: &Stmts, scope_depth: usize) -> Result<String, Feedback> {
		let mut res = String::new();

		for stmt in &stmts.stmts {
			match self.eval_stmt(stmt, scope_depth) {
				Ok(x) => res.push_str(&x),
				Err(e) => return Err(e)
			}
		}

		Ok(res)
	}

	fn eval_unary_expr(&mut self, unary_expr: &UnaryExpr, scope_depth: usize) -> Result<String, Feedback> {
		Ok(format!("{}{}", unary_expr.op, self.eval_expr(&unary_expr.val, scope_depth)?))
	}

	fn eval_var_decl(&mut self, var_decl: &VarDecl, scope_depth: usize) -> Result<String, Feedback> {
		let scope_tabs = "\t".repeat(scope_depth);

		let var_type = if let Some(var_type) = &var_decl.var_type {
			var_type.to_owned()
		} else {
			if let Some(val) = &var_decl.val {
				self.type_infer(val)?
			} else{
				todo!("Infer after declaration is not supported yet");
			}
		};

		if let Some(val) = &var_decl.val {
			Ok(format!("{scope_tabs}{} {} = {};\n", var_type, var_decl.id, self.eval_expr(val, scope_depth)?))
		} else {
			Ok(format!("{scope_tabs}{} {};\n", var_type, var_decl.id))
		}
	}
}

pub fn transpile(ast: &Node) -> Result<String, Feedback> {
	let mut transpiler = Transpiler::new();

	if let Node::Program(program) = ast {
		let mut source = transpiler.eval_program(program)?;

		if transpiler.found_main() {
			source.push_str("\
\n
int main(int argc, char** argv) {
	main_();
	return 0;
}\
			");
		}

		Ok(source)
	} else {
		Err(Error::unspecified("Expected Program"))
	}
}