use bird_parser::*;
use bird_parser::patterns::*;
use bird_utils::*;

pub struct Transpiler;

impl Transpiler {
	fn new() -> Self {
		Self
	}

	fn eval_program(&mut self, program: &Program) -> Result<String, Feedback> {
		self.eval_stmts(&program.stmts)
	}

	fn eval_extern_block(&mut self, extern_block: &ExternBlock) -> Result<String, Feedback> {
		if extern_block.lang == "\"C\"" {
			self.eval_items(&extern_block.items)
		} else {
			Err(Error::unspecified("Extern blocks only support the C language"))
		}
	}

	fn eval_func(&mut self, func: &Func) -> Result<String, Feedback> {
		let id = if func.id == "main" {
			"main_".to_owned()
		} else {
			func.id.to_owned()
		};

		Ok(format!("void {}(void);\n", id))
	}

	fn eval_func_proto(&mut self, func_proto: &FuncProto) -> Result<String, Feedback> {
		Ok(format!("void {}(void);\n", func_proto.id))
	}

	fn eval_item(&mut self, item: &Item) -> Result<String, Feedback> {
		match item {
			Item::ExternBlock(x) => self.eval_extern_block(x),
			Item::Func(x) => self.eval_func(x),
			Item::FuncProto(x) => self.eval_func_proto(x),
			Item::VarDecl(x) => self.eval_var_decl(x)
		}
	}

	fn eval_items(&mut self, items: &Items) -> Result<String, Feedback> {
		let mut res = String::new();

		for item in &items.items {
			res.push_str(&self.eval_item(item)?)
		}

		Ok(res)
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

	fn eval_var_decl(&mut self, var_decl: &VarDecl) -> Result<String, Feedback> {
		let var_type = match &var_decl.var_type {
			Some(x) => x.to_owned(),
			None => return Err(Error::invalid_syntax(Some(&var_decl.loc), "Cannot infer type of a global variable"))
		};

		Ok(format!("{} {};", var_type, var_decl.id))
	}
}

pub fn transpile(ast: &Node) -> Result<String, Feedback> {
	let mut transpiler = Transpiler::new();

	if let Node::Program(program) = ast {
		let upper_file_name = program.loc.filename.as_ref().unwrap().split(".").collect::<Vec<&str>>()[0].to_uppercase() + "_H";

		Ok(format!("\
#ifndef {}
#define {}

{}
#endif
		\
			", upper_file_name, upper_file_name, transpiler.eval_program(program)?))
	} else {
		Err(Error::unspecified("Expected Program"))
	}
}