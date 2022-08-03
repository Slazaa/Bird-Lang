use crate::bird::feedback::*;
use crate::bird::lexer::*;
use crate::bird::parser::parser::*;

use super::expression::*;

fn check_scope(parser: &Parser, keyword: &str) -> bool {
	match parser.parent_node() {
		Node::Program { .. } => {
			match keyword {
				"func" => true,
				_ => false
			}
		}
		_ => {
			match keyword {
				"break" | "continue" | "else" | "if" | "loop" | "return" => true,
				_ => false
			}
		}
	}
}

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

	if !check_scope(parser, current_token.symbol()) {
		return Err(Error::unexpected(current_token.pos(), &format!("'{}'", current_token.symbol())));
	}
	
	match *current_token.token_type() {
		TokenType::Keyword => {
			match current_token.symbol() {
				"break" => todo!(),
				"const" => todo!(),
				"continue" => todo!(),
				"return" => todo!(),
				"var" => return var_decl(parser),
				_ => ()
			}
		}
		TokenType::Identifier => {
			todo!();
		}
		_ => ()
	}

	control_flow_statement(parser)
}

pub fn control_flow_statement(parser: &mut Parser) -> Result<Node, Feedback> {
	let current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::expected(parser.last_token().unwrap().pos(), "token", None))
	};

	if !check_scope(parser, current_token.symbol()) {
		return Err(Error::unexpected(current_token.pos(), &format!("'{}'", current_token.symbol())));
	}

	if *current_token.token_type() == TokenType::Keyword {
		match current_token.symbol() {
			"else" => todo!(),
			"func" => return func_decl(parser),
			"if" => todo!(),
			"loop" => todo!(),
			_ => ()
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

				match memb_decl(parser) {
					Ok(param) => params.push(param),
					Err(e) => return Err(e) 
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

	let mut func_decl = Node::FuncDecl { public, identifier, params, return_type, body: None };

	match current_token.token_type() {
		TokenType::Separator if current_token.symbol() == "{" => {
			parser.advance();

			let parent_node = parser.parent_node()
				.clone();

			*parser.parent_node_mut() = func_decl.clone();

			let func_body = match parser.statements() {
				Ok(x) => x,
				Err(e) => return Err(e)
			};

			*parser.parent_node_mut() = parent_node;

			if let Node::FuncDecl { body, .. } = &mut func_decl {
				*body = Some(func_body);
			}
	
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

	match parser.parent_node() {
		Node::Program { .. } => {
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

	let mut var_decl = Node::VarDecl { identifier, var_type, value: None, public, global };

	current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	match current_token.token_type() {
		TokenType::Operator if current_token.symbol() == "=" => {
			parser.advance();

			if let Node::VarDecl { value, .. } = &mut var_decl {
				*value = match expr(parser) {
					Ok(x) => Some(Box::new(x)),
					Err(e) => return Err(e)
				};
			}
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

	Ok(Node::MembDecl { identifier, param_type })
}

pub fn identifier(parser: &mut Parser) -> Result<Node, Feedback> {
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
		TokenType::Operator if current_token.symbol() == "=" => assignment(parser, &identifier),
		TokenType::Separator if current_token.symbol() == "(" => todo!(),
		_ => expr(parser)
	}
}

// TODO
pub fn assignment(parser: &mut Parser, identifier: &str) -> Result<Node, Feedback> {
	let mut current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	todo!();
}