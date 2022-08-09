use std::fs::OpenOptions;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write as _;
use std::fmt::Write as _;

use crate::bird::constants::compile;
use crate::bird::feedback::*;
use crate::bird::parser::*;

static OUTPUT_FOLDER: &str = "c_output";

fn types_file() -> Result<(), Feedback> {
	if !Path::new(&format!("{}/bird", OUTPUT_FOLDER)).exists() && fs::create_dir(&format!("{}/bird", OUTPUT_FOLDER)).is_err() {
		return Err(Error::unspecified(&format!("Failed creating '{}/bird' directory", OUTPUT_FOLDER)));
	}

	let mut types_file = match OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(&format!("{}/bird/types.h", OUTPUT_FOLDER))
	{
		Ok(x) => x,
		Err(_) => return Err(Error::unspecified("Failed creating 'bird/types.h' file")) 
	};

	if write!(types_file, "\
#ifndef BIRD_TYPES_H
#define BIRD_TYPES_H

typedef enum {{ false, true }} bool;

typedef char int8;
typedef short int16;
typedef long int32;
typedef long long int64;

typedef unsigned char uint8;
typedef unsigned short uint16;
typedef unsigned long uint32;
typedef unsigned long long uint64;

typedef float float32;
typedef double float64;

#endif\
		").is_err() {
			return Err(Error::unspecified("Failed writing to 'bird/types.h' file"));
		}

	Ok(())
}
pub struct Compiler {
	main_file: File,
	func_protos: Vec<String>
}

impl Compiler {
	pub fn compile(ast: Node) -> Result<(), Feedback> {
		if !Path::new(OUTPUT_FOLDER).exists() && fs::create_dir(OUTPUT_FOLDER).is_err() {
			return Err(Error::unspecified(&format!("Failed creating '{}' directory", OUTPUT_FOLDER)));
		}

		let main_file = match OpenOptions::new()
			.write(true)
			.truncate(true)
			.create(true)
			.open(&format!("{}/main.c", OUTPUT_FOLDER))
		{
			Ok(x) => x,
			Err(_) => return Err(Error::unspecified("Failed creating 'main.c' file")) 
		};

		if let Err(e) = types_file() {
			return Err(e);
		}

		let mut compiler = Self {
			main_file,
			func_protos: Vec::new()
		};

		if writeln!(compiler.main_file, "#include \"bird/types.h\"").is_err() {
			return Err(Error::unspecified("Failed writing to 'main.c' file"));
		}

		match compiler.translate_ast(&ast) {
			Ok(res) => {
				for proto in compiler.func_protos {
					if write!(compiler.main_file, "{}", proto).is_err() {
						return Err(Error::unspecified("Failed writing to 'main.c' file"));
					}
				}

				if write!(compiler.main_file, "{}", res).is_err() {
					return Err(Error::unspecified("Failed writing to 'main.c' file"));
				}
			}
			Err(e) => return Err(e)
		}

		if write!(compiler.main_file, "int main(int argc, char** argv){{{}main();return 0;}}", compile::FUNC_PREFIX).is_err() {
			return Err(Error::unspecified("Failed writing to 'main.c' file"));
		}

		Ok(())
	}

	fn translate_ast(&mut self, parent_node: &Node) -> Result<String, Feedback> {
		match parent_node {
			Node::Program { .. } => self.program(parent_node),
			Node::FuncDecl { public, identifier, params, return_type, body } => self.func_decl(*public, identifier, params, return_type, body),
			Node::VarDecl { public, global, identifier, var_type, value } => {
				let value = value.as_ref().map(|x| (**x).clone());
				self.var_decl(*public, *global, identifier, var_type, &value)
			}
			Node::Assignment { identifier, operator, value } => self.assignment(identifier, operator, &*value),
			Node::FuncCall { identifier, params } => self.func_call(identifier, params),
			Node::IfStatement { condition, body } => self.if_statement(&*condition, body),
			_ => Err(Error::unspecified("Invalid node"))
		}
	}

	fn program(&mut self, node: &Node) -> Result<String, Feedback> {
		let mut res = String::new();

		if let Node::Program { body } = node {
			for node in body {
				let translated_node = self.translate_ast(node)?;
				res.push_str(&translated_node);
			}
		}

		Ok(res)
	}

	fn func_decl(&mut self, public: bool, identifier: &str, params: &Vec<Node>, return_type: &Option<String>, body: &Option<Vec<Node>>) -> Result<String, Feedback> {
		let mut res = String::new();

		match return_type {
			Some(return_type) => write!(&mut res, "{} ", return_type).unwrap(),
			None => res.push_str("void ")
		}

		write!(&mut res, "{}{}(", compile::FUNC_PREFIX, identifier).unwrap();

		if !params.is_empty() {
			for node in params {
				if let Node::MembDecl { identifier, param_type } = node {
					write!(&mut res, "{} {}, ", param_type, identifier).unwrap();
				}
			}

			res.truncate(res.len() - 2);
		} else {
			res.push_str("void");
		}

		res.push(')');
		
		if let Some(body) = body {
			let node_signature = Node::FuncDecl { public, identifier: identifier.to_owned(), params: params.to_vec(), return_type: return_type.to_owned(), body: None };
			let node_signature_string = self.translate_ast(&node_signature)?;

			self.func_protos.push(node_signature_string);

			res.push('{');

			for node in body {
				res.push_str(&self.translate_ast(node)?);
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

	fn var_decl(&mut self, public: bool, global: bool, identifier: &str, var_type: &str, value: &Option<Node>) -> Result<String, Feedback> {
		let mut res = String::new();

		if global && !public {
			res.push_str("static ");
		}
		
		write!(res, "{} ", var_type).unwrap();

		match value {
			Some(value) => {
				let value = match self.expr(value) {
					Ok(x) => x,
					Err(e) => return Err(e)
				};

				write!(&mut res, "{}={};", identifier, value).unwrap();
			}
			None => write!(&mut res, "{};", identifier).unwrap()
		}

		Ok(res)
	}

	fn expr(&mut self, node: &Node) -> Result<String, Feedback> {
		match node {
			Node::Literal(value) => Ok(value.to_owned()),
			_ => todo!()
		}
	}

	fn assignment(&mut self, identifier: &str, operator: &str, value: &Node) -> Result<String, Feedback> {
		let mut res = String::new();

		let value = match self.expr(value) {
			Ok(x) => x,
			Err(e) => return Err(e)
		};

		write!(res, "{}{}{};", identifier, operator, value).unwrap();

		Ok(res)
	}

	fn func_call(&mut self, identifier: &str, params: &Vec<Node>) -> Result<String, Feedback> {
		let mut res = String::new();

		write!(&mut res, "{}{}(", compile::FUNC_PREFIX, identifier).unwrap();

		if !params.is_empty() {
			for node in params {
				if let Node::Identifier(identifier) = node {
					write!(&mut res, "{}, ", identifier).unwrap();
				}
			}

			res.truncate(res.len() - 2);
		}

		res.push_str(");");

		Ok(res)
	}

	fn if_statement(&mut self, condition: &Node, body: &Vec<Node>) -> Result<String, Feedback> {
		let mut res = String::new();

		let condition = match self.expr(condition) {
			Ok(x) => x,
			Err(e) => return Err(e)
		};

		write!(&mut res, "if({}){{", condition).unwrap();

		for node in body {
			res.push_str(&self.translate_ast(node)?);
		}

		res.push('}');

		Ok(res)
	}
}