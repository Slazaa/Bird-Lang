pub mod array;
pub mod types;

use std::fs::OpenOptions;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write as _;
use std::fmt::Write as _;

use crate::bird::constants::compile;
use crate::bird::feedback::*;
use crate::bird::parser::*;
use crate::bird::SRC_FOLDER;

pub static OUTPUT_FOLDER: &str = "c";

pub struct Compiler {
	main_file: File,
	func_protos: Vec<String>
}

impl Compiler {
	pub fn compile(ast: &Node, file_path: &Path) -> Result<(), Feedback> {
		let mut output = file_path.to_path_buf();
		let parent_folder = output.parent()
			.ok_or_else(|| Error::invalid_syntax(None, "Invalid path"))?;

		if parent_folder.to_str().unwrap() == SRC_FOLDER {
			let filename = file_path.file_name().unwrap().to_str().unwrap();
			output = Path::new(filename).to_path_buf();
		}

		output = Path::new(OUTPUT_FOLDER).join(output);
		output.set_extension("c");

		{
			let parent_folder = output.parent()
				.ok_or_else(|| Error::invalid_syntax(None, "Invalid path"))?;

			if !Path::new(parent_folder).exists() && fs::create_dir_all(parent_folder).is_err() {
				return Err(Error::unspecified(&format!("Failed creating '{}' directory", parent_folder.display())));
			}
		}

		let main_file = match OpenOptions::new()
			.write(true)
			.truncate(true)
			.create(true)
			.open(output)
		{
			Ok(x) => x,
			Err(_) => return Err(Error::unspecified("Failed creating 'main.c' file")) 
		};

		let mut compiler = Self {
			main_file,
			func_protos: Vec::new()
		};

		if writeln!(compiler.main_file, "#include \"bird/types.h\"").is_err() {
			return Err(Error::unspecified("Failed writing to 'main.c' file"));
		}

		let result = compiler.eval(ast)?;

		for proto in compiler.func_protos {
			if write!(compiler.main_file, "{}", proto).is_err() {
				return Err(Error::unspecified("Failed writing to 'main.c' file"));
			}
		}

		if write!(compiler.main_file, "{}int main(int argc, char** argv){{{}main();return 0;}}", result, compile::FUNC_PREFIX).is_err() {
			return Err(Error::unspecified("Failed writing to 'main.c' file"));
		}

		Ok(())
	}

	fn eval(&mut self, node: &Node) -> Result<String, Feedback> {
		match node {
			Node::Literal(value, ..) => Ok(value.to_owned()),
			Node::Identifier(value, ..) => Ok(value.to_owned()),
			Node::Operator(value, ..) => Ok(value.to_owned()),

			Node::Program { body } => self.program(body),

			Node::UnaryExpr { operator, node } => self.unary_expr(operator, node),
			Node::BinExpr { operator, left, right } => self.bin_expr(operator, left, right),

			Node::FuncDecl { public, identifier, params, return_type, body } => self.func_decl(*public, identifier, params, return_type, body),
			Node::VarDecl { public, global, identifier, var_type, value } => self.var_decl(*public, *global, identifier, var_type, value),

			Node::Assignment { identifier, operator, value } => self.assignment(identifier, operator, &*value),

			Node::FuncCall { identifier, params } => self.func_call(identifier, params),

			Node::IfStatement { condition, body } => self.if_statement(condition, body),
			Node::LoopStatement { condition, body } => self.loop_statement(condition, body),

			Node::Type { identifier } => self.type_node(identifier),
			Node::TypePtr { identifier, mutable } => self.type_ptr_node(identifier, *mutable),
			_ => todo!()
		}
	}

	fn program(&mut self, body: &Vec<Node>) -> Result<String, Feedback> {
		let mut res = String::new();

		for node in body {
			let translated_node = self.eval(node)?;
			res.push_str(&translated_node);
		}

		Ok(res)
	}

	fn unary_expr(&mut self, operator: &Node, node: &Node) -> Result<String, Feedback> {
		Ok(format!("{}{}", self.eval(operator)?, self.eval(node)?))
	}

	fn bin_expr(&mut self, operator: &Node, left: &Node, right: &Node) -> Result<String, Feedback> {
		Ok(format!("{}{}{}", self.eval(left)?, self.eval(operator)?, self.eval(right)?))
	}

	fn func_decl(&mut self, public: bool, identifier: &Node, params: &Vec<(Node, Node)>, return_type: &Option<Node>, body: &Option<Vec<Node>>) -> Result<String, Feedback> {
		let mut res = String::new();

		match return_type {
			Some(return_type) => write!(&mut res, "{} ", self.eval(return_type)?).unwrap(),
			None => res.push_str("void ")
		}

		write!(&mut res, "{}(", self.eval(identifier)?).unwrap();

		if !params.is_empty() {
			for (identifier, var_type) in params {
				write!(&mut res, "{} {}, ", self.eval(var_type)?, self.eval(identifier)?).unwrap();
			}

			res.truncate(res.len() - 2);
		} else {
			res.push_str("void");
		}

		res.push(')');
		
		if let Some(body) = body {
			let node_signature = Node::FuncDecl { public, identifier: Box::new(identifier.to_owned()), params: params.to_vec(), return_type: Box::new(return_type.to_owned()), body: None };
			let node_signature_string = self.eval(&node_signature)?;

			self.func_protos.push(node_signature_string);

			res.push('{');

			for node in body {
				res.push_str(&self.eval(node)?);
			}

			res.push('}');
		} else {
			if !public {
				res = "static ".to_owned() + &res;
			}

			res.push(';');
		}

		Ok(res)
	}

	fn var_decl(&mut self, public: bool, global: bool, identifier: &Node, var_type: &Node, value: &Option<Node>) -> Result<String, Feedback> {
		let mut res = String::new();

		if global && !public {
			res.push_str("static ");
		}
		
		match var_type {
			Node::TypeArray { identifier, size } => write!(res, "{} {}[{}]", self.eval(identifier)?, self.eval(identifier)?, self.eval(size)?).unwrap(),
			_ => write!(res, "{} {}", self.eval(var_type)?, self.eval(identifier)?).unwrap()
		}

		if let Some(value) = value {
			write!(&mut res, "={}", self.eval(value)?).unwrap();
		}

		res.push(';');

		Ok(res)
	}

	fn assignment(&mut self, identifier: &Node, operator: &Node, value: &Node) -> Result<String, Feedback> {
		Ok(format!("{}{}{};", self.eval(identifier)?, self.eval(operator)?, self.eval(value)?))
	}

	fn func_call(&mut self, identifier: &Node, params: &Vec<Node>) -> Result<String, Feedback> {
		let mut res = String::new();

		write!(&mut res, "{}(", self.eval(identifier)?).unwrap();

		if !params.is_empty() {
			for node in params {
				write!(&mut res, "{}, ", self.eval(node)?).unwrap();
			}

			res.truncate(res.len() - 2);
		}

		res.push_str(");");

		Ok(res)
	}

	fn if_statement(&mut self, condition: &Node, body: &Vec<Node>) -> Result<String, Feedback> {
		let mut res = String::new();

		write!(&mut res, "if({}){{", self.eval(condition)?).unwrap();

		for node in body {
			res.push_str(&self.eval(node)?);
		}

		res.push('}');

		Ok(res)
	}

	fn loop_statement(&mut self, condition: &Node, body: &Vec<Node>) -> Result<String, Feedback> {
		let mut res = String::new();

		write!(&mut res, "while({}){{", self.eval(condition)?).unwrap();

		for node in body {
			res.push_str(&self.eval(node)?);
		}

		res.push('}');

		Ok(res)
	}

	fn type_node(&mut self, identifier: &Node) -> Result<String, Feedback> {
		self.eval(identifier)
	}

	fn type_ptr_node(&mut self, identifier: &Node, mutable: bool) -> Result<String, Feedback> {
		match mutable {
			true => Ok(format!("{}*", self.eval(identifier)?)),
			false => Ok(format!("const {}*", self.eval(identifier)?))
		}
	}
}