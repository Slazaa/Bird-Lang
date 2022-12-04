use bird_parser::*;
use bird_parser::patterns::*;
use bird_utils::*;

use super::utils::*;

pub struct Transpiler;

impl Transpiler {
	fn new() -> Self {
		Self
	}

	fn eval_const_decl(&mut self, const_decl: &ConstDecl) -> Result<String, Feedback> {
		if let Some(public) = const_decl.public {
			if !public {
				return Ok("".to_owned())
			}
		}

		Ok(format!("extern {} const {};\n", type_infer(&const_decl.val)?, const_decl.id))
	}

	fn eval_extern_block(&mut self, extern_block: &ExternBlock) -> Result<String, Feedback> {
		if extern_block.lang == "\"C\"" {
			self.eval_stmts(&extern_block.stmts)
		} else {
			Err(Error::unspecified("Extern blocks only support the C language"))
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

	fn eval_func(&mut self, func: &Func) -> Result<String, Feedback> {
		if let Some(public) = func.public {
			if !public {
				return Ok("".to_owned())
			}
		}

		Ok(format!("void {}(void);\n", func.id.to_owned()))
	}

	fn eval_func_proto(&mut self, func_proto: &FuncProto) -> Result<String, Feedback> {
		if let Some(public) = func_proto.public {
			if !public {
				return Ok("".to_owned())
			}
		}

		Ok(format!("void {}(void);\n", func_proto.id))
	}

	fn eval_item(&mut self, item: &Item) -> Result<String, Feedback> {
		match item {
			Item::ConstDecl(x) => self.eval_const_decl(x),
			Item::Func(x) => {
				if &x.id == "main" {
					return Ok("".to_owned());
				}

				self.eval_func(x)
			},
			Item::FuncProto(x) => self.eval_func_proto(x),
			Item::Struct(x) => self.eval_struct(x),
			Item::VarDecl(x) => self.eval_var_decl(x)
		}
	}
	
	fn eval_program(&mut self, program: &Program) -> Result<String, Feedback> {
		self.eval_stmts(&program.stmts)
	}

	fn eval_stmt(&mut self, stmt: &Stmt) -> Result<String, Feedback> {
		match stmt {
			Stmt::Item(x) => self.eval_item(x),
			_ => Ok("".to_owned())
		}
	}

	fn eval_stmts(&mut self, stmts: &Stmts) -> Result<String, Feedback> {
		let mut res = String::new();

		for stmt in &stmts.stmts {
			res.push_str(&self.eval_stmt(stmt)?)
		}

		Ok(res)
	}

	fn eval_struct(&mut self, struct_: &Struct) -> Result<String, Feedback> {
		Ok(format!("\
typedef struct {{
{}\
}} {};\n\
		", self.eval_fields(&struct_.fields)?, struct_.id))
	}

	fn eval_type(&mut self, type_: &Type) -> Result<String, Feedback> {
		match type_.ptr_kind {
			PtrKind::None => Ok(format!("{}", type_.id)),
			PtrKind::Const => Ok(format!("const {}*", type_.id)),
			PtrKind::Mut => Ok(format!("{}*", type_.id))
		}
	}

	fn eval_var_decl(&mut self, var_decl: &VarDecl) -> Result<String, Feedback> {
		Ok(match &var_decl.val {
			Some(val) => format!("extern {} {}", type_infer(val)?, var_decl.id),
			None => format!("extern void {};", var_decl.id)
		})
	}
}

pub fn transpile(ast: &Node) -> Result<String, Feedback> {
	let mut transpiler = Transpiler::new();

	if let Node::Program(program) = ast {
		let upper_file_name = rem_ext(program.loc.filename.as_ref().unwrap()).to_uppercase();

		Ok(format!("\
#ifndef {}_H
#define {}_H

{}
#endif\
			", upper_file_name, upper_file_name, transpiler.eval_program(program)?))
	} else {
		Err(Error::unspecified("Expected Program"))
	}
}