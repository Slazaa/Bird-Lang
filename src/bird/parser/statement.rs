use crate::bird::feedback::*;
use crate::bird::lexer::*;
use crate::bird::parser::parser::*;

use super::expression::*;

pub fn statement(parser: &mut Parser) -> Result<Node, Feedback> {
	let current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::expected(parser.last_token().unwrap().pos(), "token", None))
	};

	if let Some(next_pub) = parser.next_pub() {
		match current_token.symbol() {
			"func" | "struct" => (),
			_ => return Err(Error::expected(next_pub.pos(), "item", Some(current_token.symbol())))
		}
	}

	let mut to_call: Option<NodeFunc> = None;
	
	match *current_token.token_type() {
		TokenType::Keyword => {
			match current_token.symbol() {
				"break" => todo!(),
				"const" => todo!(),
				"continue" => todo!(),
				"return" => todo!(),
				"var" => to_call = Some(var_decl),
				_ => ()
			}
		}
		TokenType::Identifier => {
			todo!();
		}
		_ => ()
	}

	if let Some(to_call) = to_call {
		match parser.parent_node_item() {
			NodeItem::Array(name) if name == "Program" => {
				match current_token.symbol() {
					"break" | "continue" | "return" => return Err(Error::unexpected(current_token.pos(), &format!("'{}'", current_token.symbol()))),
					_ => return to_call(parser)
				}
			}
			_ => ()
		}
	}

	control_flow_statement(parser)
}

pub fn control_flow_statement(parser: &mut Parser) -> Result<Node, Feedback> {
	let current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::expected(parser.last_token().unwrap().pos(), "token", None))
	};

	let mut to_call: Option<NodeFunc> = None;

	if *current_token.token_type() == TokenType::Keyword {
		match current_token.symbol() {
			"else" => todo!(),
			"func" => to_call = Some(func_decl),
			"if" => todo!(),
			"loop" => todo!(),
			_ => ()
		}
	}

	if let Some(to_call) = to_call {
		if let NodeItem::Array(name) = parser.parent_node_item() {
			if name == "Program" {
				match current_token.symbol() {
					"else" | "if" | "loop" => (),
					_ => return to_call(parser)
				}
			} else {
				if current_token.symbol() != "func" {
					return to_call(parser);
				}
			}

			return Err(Error::unexpected(current_token.pos(), &format!("'{}'", current_token.symbol())));
		}
	}

	Err(Error::expected(current_token.pos(), "statement", Some(&format!("'{}'", current_token.symbol()))))
}

pub fn func_decl(parser: &mut Parser) -> Result<Node, Feedback> {
	let public = match parser.next_pub() {
		Some(_) => {
			*parser.next_pub_mut() = None;
			true
		},
		None => false
	};

	let mut current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	let identifier = match current_token.token_type() {
		TokenType::Identifier => current_token.symbol().to_owned(),
		_ => return Err(Error::expected(current_token.pos(), "'Identifier'", Some(&format!("'{}'", current_token.symbol()))))
	};

	parser.advance();
	parser.skip_new_lines();

	current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	match current_token.token_type() {
		TokenType::Separator if current_token.symbol() == "(" => (),
		_ => return Err(Error::expected(current_token.pos(), "'('", Some(&format!("'{}'", current_token.symbol()))))
	}

	parser.advance();
	parser.skip_new_lines();

	let mut params = Vec::new();

	current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	match current_token.token_type() {
		TokenType::Separator if current_token.symbol() == ")" => (),
		_ => {
			loop {
				parser.skip_new_lines();

				let param_entry = match memb_decl(parser) {
					Ok(param) => param.entry().clone(),
					Err(e) => return Err(e) 
				};
	
				if let NodeItem::MembDecl { identifier, param_type } = param_entry {
					params.push((identifier, param_type));
				}
	
				parser.advance();
				parser.skip_new_lines();

				current_token = match parser.current_token() {
					Some(x) => x.clone(),
					None => return Err(Error::expected(current_token.pos(), "',' or ')'", None))
				};
	
				match current_token.symbol() {
					"," => parser.advance(),
					")" => break,
					_ => return Err(Error::expected(current_token.pos(), "',' or ')'", Some(&format!("'{}'", current_token.symbol()))))
				};
			}
		}
	}

	current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	let return_type = match current_token.token_type() {
		TokenType::Operator if current_token.symbol() == "->" => {
			current_token = match parser.advance() {
				Some(x) => x.clone(),
				None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
			};

			parser.advance();

			match current_token.token_type() {
				TokenType::Identifier => Some(current_token.symbol().to_owned()),
				_ => return Err(Error::expected(current_token.pos(), "'Identifier'", Some(&format!("'{}'", current_token.symbol()))))
			}
		}
		_ => None
	};

	parser.skip_new_lines();

	current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::expected(parser.last_token().unwrap().pos(), "token", None))
	};

	let mut func_decl = Node::new(NodeItem::FuncDecl { identifier, params, return_type, public }, vec![]);

	match current_token.token_type() {
		TokenType::Separator if current_token.symbol() == "{" => {
			parser.advance();

			let parent_node_item = parser.parent_node_item()
				.clone();

			let func_body = match parser.statements("Body") {
				Ok(x) => x,
				Err(e) => return Err(e)
			};

			*parser.parent_node_item_mut() = parent_node_item;
	
			func_decl.children_mut()
				.push(func_body);
	
			current_token = match parser.current_token() {
				Some(x) => x.clone(),
				None => return Err(Error::expected(parser.last_token().unwrap().pos(), "token", None))
			};
	
			match current_token.token_type() {
				TokenType::Separator if current_token.symbol() == "}" => (),
				_ => return Err(Error::expected(current_token.pos(), "'}'", Some(&format!("'{}'", current_token.symbol()))))
			}
		}
		_ => return Ok(func_decl)
	}

	parser.advance();

	Ok(func_decl)
}

pub fn var_decl(parser: &mut Parser) -> Result<Node, Feedback> {
	let mut public = false;
	let mut global = false;

	match parser.parent_node_item() {
		NodeItem::Array(name) if name == "Program" => {
			if parser.next_pub().is_some() {
				*parser.next_pub_mut() = None;
				public = true;
			}

			global = true;
		}
		_ => ()
	}

	let mut current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	let identifier = match current_token.token_type() {
		TokenType::Identifier => current_token.symbol().to_owned(),
		_ => return Err(Error::expected(current_token.pos(), "'Identifier'", Some(&format!("'{}'", current_token.symbol()))))
	};

	current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	match current_token.token_type() {
		TokenType::Operator if current_token.symbol() == ":" => (),
		_ => return Err(Error::expected(current_token.pos(), "':'", Some(&format!("'{}'", current_token.symbol()))))
	};

	current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	let var_type = match current_token.token_type() {
		TokenType::Identifier => current_token.symbol().to_owned(),
		_ => return Err(Error::expected(current_token.pos(), "'Identifier'", Some(&format!("'{}'", current_token.symbol()))))
	};

	let mut var_decl = Node::new(NodeItem::VarDecl { identifier, var_type, public, global }, vec![]);

	current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	match current_token.token_type() {
		TokenType::Operator if current_token.symbol() == "=" => {
			parser.advance();

			let expr = match expr(parser) {
				Ok(x) => x,
				Err(e) => return Err(e)
			};

			var_decl.children_mut()
				.push(expr);
		}
		_ => ()
	}

	Ok(var_decl)
}

pub fn memb_decl(parser: &mut Parser) -> Result<Node, Feedback> {
	let mut current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	let identifier = match current_token.token_type() {
		TokenType::Identifier => current_token.symbol().to_owned(),
		_ => return Err(Error::expected(current_token.pos(), "'Identifier'", Some(&format!("'{}'", current_token.symbol()))))
	};

	current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	match current_token.token_type() {
		TokenType::Operator if current_token.symbol() == ":" => (),
		_ => return Err(Error::expected(current_token.pos(), "':'", Some(&format!("'{}'", current_token.symbol()))))
	}

	current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	let param_type = match current_token.token_type() {
		TokenType::Identifier => current_token.symbol().to_owned(),
		_ => return Err(Error::expected(current_token.pos(), "':'", Some(&format!("'{}'", current_token.symbol()))))
	};

	Ok(Node::new(NodeItem::MembDecl { identifier, param_type }, vec![]))
}