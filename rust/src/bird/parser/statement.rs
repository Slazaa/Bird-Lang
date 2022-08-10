use crate::bird::feedback::*;
use crate::bird::lexer::*;
use crate::bird::parser::parse::*;

use super::expression::*;

/// Checks if the current `Token` is allowed in the current scope.
fn check_scope(parser: &Parser) -> bool {
	let current_token = match parser.current_token() {
		Some(x) => x,
		None => return false
	};

	match parser.parent_node() {
		Node::Program { .. } => {
			match current_token.token_type() {
				TokenType::Keyword => matches!(current_token.symbol(), "func"),
				_ => false
			}
		}
		_ => {
			match current_token.token_type() {
				TokenType::Keyword => matches!(
					current_token.symbol(),
					"break"  | "continue" | "const" |
					"else"   | "func"     | "if"    |
					"loop"   | "return"   | "var"
				),
				TokenType::Identifier => true,
				_ => false
			}
		}
	}
}

/// Evaluates a statement.
pub fn statement(parser: &mut Parser) -> Result<Node, Feedback> {
	let current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::expected(parser.last_token().unwrap().pos(), "token", None))
	};

	if let Some(next_pub) = parser.next_pub() {
		match current_token.symbol() {
			"func" => (),
			_ => return Err(Error::expected(next_pub.pos(), "item", Some(current_token.symbol())))
		}
	}

	if !check_scope(parser) {
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
		TokenType::Identifier => return assignment(parser),
		_ => ()
	}

	control_flow_statement(parser)
}

/// Evaluates a control flow statement.
pub fn control_flow_statement(parser: &mut Parser) -> Result<Node, Feedback> {
	let current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::expected(parser.last_token().unwrap().pos(), "token", None))
	};

	if !check_scope(parser) {
		return Err(Error::unexpected(current_token.pos(), &format!("'{}'", current_token.symbol())));
	}

	if *current_token.token_type() == TokenType::Keyword {
		match current_token.symbol() {
			"else" => todo!(),
			"func" => return func_decl(parser),
			"if" => return if_statement(parser),
			"loop" => todo!(),
			_ => ()
		}
	}

	Err(Error::expected(current_token.pos(), "statement", Some(&format!("'{}'", current_token.symbol()))))
}

/// Creates a `Node::FuncDecl`.
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
		TokenType::Identifier => Node::Identifier(current_token.symbol().to_owned()),
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

				let (identifier, var_type) = var_def(parser)?;
				let var_type = match var_type {
					Some(x) => x,
					None => return Err(Error::expected(current_token.pos(), "'Type'", Some(&format!("'{}'", current_token.symbol()))))
				};

				params.push((identifier, var_type));
	
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
				TokenType::Identifier => {
					let return_type = Node::Identifier(current_token.symbol().to_owned());
					Some(return_type)
				},
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

	let mut func_decl = Node::FuncDecl { public, identifier: Box::new(identifier), params, return_type: Box::new(return_type), body: None };

	match current_token.token_type() {
		TokenType::Separator if current_token.symbol() == "{" => {
			parser.advance();

			let parent_node = parser.parent_node()
				.clone();

			*parser.parent_node_mut() = func_decl.clone();

			let func_body = parser.statements()?;

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

pub fn var_def(parser: &mut Parser) -> Result<(Node, Option<Node>), Feedback> {
	let mut current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	let identifier = match current_token.token_type() {
		TokenType::Identifier => Node::Identifier(current_token.symbol().to_owned()),
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
		TokenType::Identifier => Some(Node::Identifier(current_token.symbol().to_owned())),
		_ => None
	};

	return Ok((identifier, var_type))
}

/// Creates a `Node::VarDecl`.
pub fn var_decl(parser: &mut Parser) -> Result<Node, Feedback> {
	let mut public = false;
	let mut global = false;

	if let Node::Program { .. } = parser.parent_node() {
		if parser.next_pub().is_some() {
			*parser.next_pub_mut() = None;
			public = true;
		}

		global = true;
	}

	let (identifier, var_type) = var_def(parser)?;

	let mut current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	let var_type = match var_type {
		Some(x) => x,
		None => return Err(Error::expected(current_token.pos(), "'Type'", Some(&format!("'{}'", current_token.symbol()))))
	};

	let mut var_decl = Node::VarDecl { identifier: Box::new(identifier), var_type: Box::new(var_type), value: Box::new(None), public, global };

	current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	match current_token.token_type() {
		TokenType::Operator if current_token.symbol() == "=" => {
			parser.advance();

			if let Node::VarDecl { value, .. } = &mut var_decl {
				*value = Box::new(Some(expr(parser)?));
			}
		}
		_ => ()
	}

	Ok(var_decl)
}

/// Creates a `Node::MembDecl`.
/*
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
*/
pub fn assignment(parser: &mut Parser) -> Result<Node, Feedback> {
	let mut current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	let identifier = match current_token.token_type() {
		TokenType::Identifier => Node::Identifier(current_token.symbol().to_owned()),
		_ => return Err(Error::expected(current_token.pos(), "'Identifier'", Some(&format!("'{}'", current_token.symbol()))))
	};

	current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	match current_token.symbol() {
		"(" => return func_call(parser, &identifier),
		"=" | "+=" | "-=" | "*=" | "/=" | "%=" | "<<=" | ">>=" | "&=" | "^=" | "|=" => (),
		_ => return Err(Error::expected(current_token.pos(), "'(' or '='", Some(&format!("'{}'", current_token.symbol()))))
	}

	let operator = Node::Operator(current_token.symbol().to_owned());

	parser.advance();

	let value = expr(parser)?;

	Ok(Node::Assignment { identifier: Box::new(identifier), operator: Box::new(operator), value: Box::new(value) })
}

/// Creates a `Node::FuncCall`.
pub fn func_call(parser: &mut Parser, identifier: &Node) -> Result<Node, Feedback> {
	let mut current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	let mut params = Vec::new();

	match current_token.token_type() {
		TokenType::Separator if current_token.symbol() == ")" => (),
		_ => {
			loop {
				parser.skip_new_lines();

				match current_token.token_type() {
					TokenType::Identifier => params.push(Node::Identifier(current_token.symbol().to_owned())),
					_ => return Err(Error::expected(current_token.pos(), "'Identifier'", Some(&format!("'{}'", current_token.symbol()))))
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

	parser.advance();

	Ok(Node::FuncCall { identifier: Box::new(identifier.clone()), params })
}

pub fn if_statement(parser: &mut Parser) -> Result<Node, Feedback> {
	let mut current_token = match parser.advance() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	if current_token.symbol() != "(" {
		return Err(Error::expected(current_token.pos(), "(", Some(&format!("'{}'", current_token.symbol()))));
	}

	let condition = expr(parser)?;

	parser.advance();
	parser.skip_new_lines();

	current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	if current_token.symbol() != "{" {
		return Err(Error::expected(current_token.pos(), "{", Some(&format!("'{}'", current_token.symbol()))));
	}

	parser.advance();

	let parent_node = parser.parent_node()
		.clone();

	let mut if_statement = Node::IfStatement { condition: Box::new(condition), body: Vec::new() };

	*parser.parent_node_mut() = if_statement.clone();

	let if_body = parser.statements()?;

	*parser.parent_node_mut() = parent_node;

	if let Node::IfStatement { body, .. } = &mut if_statement {
		*body = if_body;
	}

	current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::expected(parser.last_token().unwrap().pos(), "token", None))
	};

	match current_token.token_type() {
		TokenType::Separator if current_token.symbol() == "}" => (),
		_ => return Err(Error::expected(current_token.pos(), "'}'", Some(&format!("'{}'", current_token.symbol()))))
	}

	parser.advance();

	Ok(if_statement)
}