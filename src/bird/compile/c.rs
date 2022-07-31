use std::fs::OpenOptions;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write as _;
use std::fmt::Write as _;

use crate::bird::constants::compile;
use crate::bird::parser::*;
use crate::bird::feedback::*;

fn types_file() -> Result<(), Feedback> {
	if !Path::new("c/bird").exists() && fs::create_dir("c/bird").is_err() {
		return Err(Error::unspecified("Failed creating 'c/bird' directory"));
	}

	let mut types_file = match OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open("c/bird/types.h")
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
		if !Path::new("c").exists() && fs::create_dir("c").is_err() {
			return Err(Error::unspecified("Failed creating 'c' directory"));
		}

		let main_file = match OpenOptions::new()
			.write(true)
			.truncate(true)
			.create(true)
			.open("c/main.c")
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

		if write!(compiler.main_file, "int main(int argc, char **argv){{{}main();return 0;}}", compile::FUNC_PREFIX).is_err() {
			return Err(Error::unspecified("Failed writing to 'main.c' file"));
		}

		Ok(())
	}

	fn translate_ast(&mut self, node: &Node) -> Result<String, Feedback> {
		match node.entry() {
			NodeItem::Array(_) => self.array(node),
			NodeItem::FuncDecl { identifier, params, return_type, public } => self.func_decl(identifier, params, return_type, *public, node),
			NodeItem::VarDecl { identifier, var_type, public, global } => self.var_decl(identifier, var_type, *public, *global, node),
			_ => Err(Error::unspecified("Invalid node"))
		}
	}

	fn array(&mut self, node: &Node) -> Result<String, Feedback> {
		let mut res = String::new();

		for node in node.children() {
			let translated_node = self.translate_ast(node)?;
			res.push_str(&translated_node);
		}

		Ok(res)
	}

	fn func_decl(&mut self, identifier: &str, params: &Vec<(String, String)>, return_type: &Option<String>, public: bool, node: &Node) -> Result<String, Feedback> {
		let mut res = String::new();

		match return_type {
			Some(return_type) => write!(&mut res, "{} ", return_type).unwrap(),
			None => res.push_str("void ")
		}

		write!(&mut res, "{}{}(", compile::FUNC_PREFIX, identifier).unwrap();

		if !params.is_empty() {
			for (param_id, param_type) in params {
				write!(&mut res, "{} {}, ", param_type, param_id).unwrap();
			}

			res.truncate(res.len() - 2);
		} else {
			res.push_str("void");
		}

		res.push(')');
		
		if let Some(body) = node.children()
			.iter()
			.find(|node| matches!(node.entry(), NodeItem::Array(name) if name == "Body"))
		{
			let mut node_signature = Node::new(node.entry().clone(), vec![]);
			node_signature.children_mut().clear();

			let node_signature_string = self.translate_ast(&node_signature)?;
			self.func_protos.push(node_signature_string);

			res.push('{');

			for node in body.children() {
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

	fn var_decl(&mut self, identifier: &str, var_type: &str, public: bool, global: bool, node: &Node) -> Result<String, Feedback> {
		let mut res = String::new();

		if global && !public {
			res.push_str("static ");
		}
		
		write!(res, "{} ", var_type).unwrap();

		match node.children().is_empty() {
			true => {
				if write!(&mut res, "{};", identifier).is_err() {
					return Err(Error::unspecified("Failed to write on result"));
				}
			}
			false => {
				let expr = match self.expr(&node.children()[0]) {
					Ok(x) => x,
					Err(e) => return Err(e)
				};

				write!(&mut res, "{}={};", identifier, expr).unwrap();
			}
		}

		Ok(res)
	}

	fn expr(&mut self, node: &Node) -> Result<String, Feedback> {
		match node.entry() {
			NodeItem::Operator(operator) => self.operator(operator, node),
			NodeItem::Literal(value) => Ok(value.clone()),
			_ => todo!()
		}
	}

	fn operator(&mut self, operator: &str, node: &Node) -> Result<String, Feedback> {
		let mut res = String::new();

		let mut get_val = |index: usize| {
			match node.children()[index].entry() {
				NodeItem::Literal(value) => Ok(value.clone()),
				NodeItem::Operator(operator) => {
					match self.operator(operator, &node.children()[0]) {
						Ok(x) => Ok(x),
						Err(e) => Err(e)
					}
				}
				_ => Err(Error::unspecified("Invalid node"))
			}
		};

		match node.children().len() {
			1 => {
				let first = match get_val(0) {
					Ok(x) => x,
					Err(e) => return Err(e)
				};

				write!(&mut res, "{}{}", operator, first).unwrap();
			}
			2 => {
				let first = match get_val(0) {
					Ok(x) => x,
					Err(e) => return Err(e)
				};

				let second = match get_val(1) {
					Ok(x) => x,
					Err(e) => return Err(e)
				};

				write!(&mut res, "{}{}{}", first, operator, second).unwrap();
			}
			_ => return Err(Error::unspecified("Invalid node"))
		}

		Ok(res)
	}
}