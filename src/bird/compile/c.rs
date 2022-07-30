use std::fs::OpenOptions;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;

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
	main_file: File
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
			main_file
		};

		if write!(compiler.main_file, "#include \"bird/types.h\"\n").is_err() {
			return Err(Error::unspecified("Failed writing to 'main.c' file"));
		}

		match compiler.translate_ast(&ast) {
			Ok(res) => {
				if write!(compiler.main_file, "{}", res).is_err() {
					return Err(Error::unspecified("Failed writing to 'main.c' file"));
				}
			}
			Err(e) => return Err(e)
		}

		if write!(compiler.main_file, "int32 main(int argc, char **argv){{{}main();return 0;}}", compile::FUNC_PREFIX).is_err() {
			return Err(Error::unspecified("Failed writing to 'main.c' file"));
		}

		Ok(())
	}

	fn translate_ast(&mut self, node: &Node) -> Result<String, Feedback> {
		match node.entry() {
			NodeItem::Array(_) => return self.array(node),
			NodeItem::FuncDecl { identifier, params, return_type, public } => return self.func_decl(identifier, params, return_type, *public, node),
			NodeItem::VarDecl { identifier, var_type, public, global } => return self.var_decl(identifier, var_type, *public, *global, node),
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

		if !public {
			res.push_str("static ");
		}

		match return_type {
			Some(return_type) => res.push_str(&format!("{} ", return_type)),
			None => res.push_str("void ")
		}

		res.push_str(&format!("{}{}(", compile::FUNC_PREFIX, identifier));

		if !params.is_empty() {
			for (param_id, param_type) in params {
				res.push_str(&format!("{} {}, ", param_type, param_id));
			}

			res.truncate(res.len() - 2);
		} else {
			res.push_str("void");
		}

		res.push_str("){");
		
		if let Some(body) = node.children()
			.iter()
			.find(|node| match node.entry() {
				NodeItem::Array(name) if name == "Body" => true,
				_ => false
			})
		{
			for node in body.children() {
				res.push_str(&self.translate_ast(node)?);
			}
		}

		res.push_str("}");

		Ok(res)
	}

	fn var_decl(&mut self, identifier: &str, var_type: &str, public: bool, global: bool, node: &Node) -> Result<String, Feedback> {
		let mut res = String::new();

		if global && !public {
			res.push_str("static ");
		}

		res.push_str(&format!("{} ", var_type));

		match node.children().is_empty() {
			true => res.push_str(&format!("{};", identifier)),
			false => {
				let expr = match self.expr(&node.children()[0]) {
					Ok(x) => x,
					Err(e) => return Err(e)
				};

				res.push_str(&format!("{}={};", identifier, expr));
			}
		}

		Ok(res)
	}

	fn expr(&mut self, node: &Node) -> Result<String, Feedback> {
		match node.entry() {
			NodeItem::Operator(operator) => return self.operator(operator, node),
			NodeItem::Literal(value) => return Ok(value.clone()),
			_ => todo!()
		}
	}

	fn operator(&mut self, operator: &str, node: &Node) -> Result<String, Feedback> {
		let mut res = String::new();

		match node.children().len() {
			1 => {
				let first = match node.children()[0].entry() {
					NodeItem::Literal(value) => value.clone(),
					NodeItem::Operator(operator) => {
						match self.operator(operator, &node.children()[0]) {
							Ok(x) => x,
							Err(e) => return Err(e)
						}
					}
					_ => return Err(Error::unspecified("Invalid node"))
				};

				res.push_str(&format!("{}{}", operator, first));
			}
			2 => {
				let first = match node.children()[0].entry() {
					NodeItem::Literal(value) => value.clone(),
					NodeItem::Operator(operator) => {
						match self.operator(operator, &node.children()[0]) {
							Ok(x) => x,
							Err(e) => return Err(e)
						}
					}
					_ => return Err(Error::unspecified("Invalid node"))
				};

				let second = match node.children()[1].entry() {
					NodeItem::Literal(value) => value.clone(),
					NodeItem::Operator(operator) => {
						match self.operator(operator, &node.children()[1]) {
							Ok(x) => x,
							Err(e) => return Err(e)
						}
					}
					_ => return Err(Error::unspecified("Invalid node"))
				};

				res.push_str(&format!("{}{}{}", first, operator, second));
			}
			_ => return Err(Error::unspecified("Invalid node"))
		}

		Ok(res)
	}
}