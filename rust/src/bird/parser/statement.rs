use crate::bird::feedback::*;
use crate::bird::lexer::*;
use crate::bird::parser::parse::*;

use super::expression::*;

/// Checks if the current `Token` is allowed in the current scope.
fn check_scope(parser: &Parser) -> bool {
	match parser.parent_node() {
		Node::Program { .. } => {
			match parser.current_token().token_type() {
				TokenType::Keyword => matches!(parser.current_token().symbol(), "func"),
				_ => false
			}
		}
		_ => {
			match parser.current_token().token_type() {
				TokenType::Keyword => matches!(
					parser.current_token().symbol(),
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

fn type_node(parser: &mut Parser) -> Result<Node, Feedback> {
	if parser.advance().is_err() {
		return Err(Error::expected(parser.current_token().pos(), "type", None))
	}

	match parser.current_token().token_type() {
		TokenType::Identifier => {
			let identifier = Node::identifier(parser.current_token().symbol(), parser.current_token().pos());
			Ok(Node::Type { identifier: Box::new(identifier) })
		}
		TokenType::Operator if parser.current_token().symbol() == "*" => {
			if parser.advance().is_err() {
				return Err(Error::expected(parser.current_token().pos(), "'mut' or type", None))
			}

			let mutable = match parser.current_token().token_type() {
				TokenType::Keyword if parser.current_token().symbol() == "mut" => {
					if parser.advance().is_err() {
						return Err(Error::expected(parser.current_token().pos(), "type", None))
					}

					true
				},
				_ => false
			};

			let identifier = match parser.current_token().token_type() {
				TokenType::Identifier => Node::identifier(parser.current_token().symbol(), parser.current_token().pos()),
				_ => return Err(Error::expected(parser.current_token().pos(), "type", Some(&format!("'{}'", parser.current_token().symbol()))))
			};

			Ok(Node::TypePtr { identifier: Box::new(identifier), mutable })
		}
		TokenType::Separator if parser.current_token().symbol() == "[" => {
			let identifier = match parser.current_token().token_type() {
				TokenType::Identifier => Node::identifier(parser.current_token().symbol(), parser.current_token().pos()),
				_ => return Err(Error::expected(parser.current_token().pos(), "type", Some(&format!("'{}'", parser.current_token().symbol()))))
			};

			match parser.current_token().token_type() {
				TokenType::Separator if parser.current_token().symbol() == "," => (),
				_ => return Err(Error::expected(parser.current_token().pos(), "','", Some(&format!("'{}'", parser.current_token().symbol()))))
			}

			let size = match parser.current_token().token_type() {
				TokenType::Literal => Node::literal(parser.current_token().symbol(), parser.current_token().pos()),
				_ => return Err(Error::expected(parser.current_token().pos(), "literal", Some(&format!("'{}'", parser.current_token().symbol()))))
			};

			match parser.current_token().token_type() {
				TokenType::Separator if parser.current_token().symbol() == "]" => (),
				_ => return Err(Error::expected(parser.current_token().pos(), "']'", Some(&format!("'{}'", parser.current_token().symbol()))))
			}

			Ok(Node::TypeArray { identifier: Box::new(identifier), size: Box::new(size) })
		}
		_ => Err(Error::expected(parser.current_token().pos(), "type", Some(&format!("'{}'", parser.current_token().symbol()))))
	}
}

/// Evaluates a statement.
pub fn statement(parser: &mut Parser) -> Result<Node, Feedback> {
	if let Some(next_pub) = parser.next_pub() {
		match parser.current_token().symbol() {
			"func" => (),
			_ => return Err(Error::unexpected(next_pub.pos(), &format!("'{}'", parser.current_token().symbol())))
		}
	}

	if !check_scope(parser) {
		return Err(Error::unexpected(parser.current_token().pos(), &format!("'{}'", parser.current_token().symbol())));
	}
	
	match parser.current_token().token_type() {
		TokenType::Keyword => {
			match parser.current_token().symbol() {
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
	if !check_scope(parser) {
		return Err(Error::unexpected(parser.current_token().pos(), &format!("'{}'", parser.current_token().symbol())));
	}

	if *parser.current_token().token_type() == TokenType::Keyword {
		match parser.current_token().symbol() {
			"else" => todo!(),
			"func" => return func_decl(parser),
			"if" => return if_statement(parser),
			"loop" => return loop_statement(parser),
			_ => ()
		}
	}

	Err(Error::expected(parser.current_token().pos(), "statement", Some(&format!("'{}'", parser.current_token().symbol()))))
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

	if parser.advance().is_err() {
		return Err(Error::expected(parser.current_token().pos(), "identifier", None))
	}

	let identifier = match parser.current_token().token_type() {
		TokenType::Identifier => {
			let identifier = parser.current_token().symbol().to_owned();
			Node::identifier(&identifier, parser.current_token().pos())
		}
		_ => return Err(Error::expected(parser.current_token().pos(), "identifier", Some(&format!("'{}'", parser.current_token().symbol()))))
	};

	if parser.advance().is_err() {
		return Err(Error::expected(parser.current_token().pos(), "'('", None))
	}

	parser.skip_new_lines();

	match parser.current_token().token_type() {
		TokenType::Separator if parser.current_token().symbol() == "(" => (),
		_ => return Err(Error::expected(parser.current_token().pos(), "'('", Some(&format!("'{}'", parser.current_token().symbol()))))
	}

	if parser.advance().is_err() {
		return Err(Error::expected(parser.current_token().pos(), "'('", None))
	}

	parser.skip_new_lines();

	let mut params = Vec::new();

	match parser.current_token().token_type() {
		TokenType::Separator if parser.current_token().symbol() == ")" => (),
		_ => {
			loop {
				parser.skip_new_lines();

				let (identifier, var_type) = var_def(parser)?;

				let var_type = match var_type {
					Some(x) => x,
					None => return Err(Error::expected(parser.current_token().pos(), "type", Some(&format!("'{}'", parser.current_token().symbol()))))
				};

				params.push((identifier, var_type));
	
				if parser.advance().is_err() {
					return Err(Error::expected(parser.current_token().pos(), "',' or ')'", None))
				}

				parser.skip_new_lines();

				match parser.current_token().symbol() {
					"," => (),
					")" => break,
					_ => return Err(Error::expected(parser.current_token().pos(), "',' or ')'", Some(&format!("'{}'", parser.current_token().symbol()))))
				};
			}
		}
	}

	let mut func_decl = Node::FuncDecl { public, identifier: Box::new(identifier), params, return_type: Box::new(None), body: None };

	if parser.advance().is_err() {
		return Err(Error::expected(parser.current_token().pos(), "'->' or '{'", None))
	}

	match parser.current_token().token_type() {
		TokenType::Operator if parser.current_token().symbol() == "->" => {
			if let Node::FuncDecl { return_type, .. } = &mut func_decl {
				*return_type = Box::new(Some(type_node(parser)?));
			}

			if parser.advance().is_err() {
				return Err(Error::expected(parser.current_token().pos(), "'->' or '{'", None))
			}
		}
		_ => ()
	};

	parser.skip_new_lines();

	match parser.current_token().token_type() {
		TokenType::Separator if parser.current_token().symbol() == "{" => {
			if parser.advance().is_err() {
				return Err(Error::expected(parser.current_token().pos(), "statement or '}'", None))
			}

			let parent_node = parser.parent_node()
				.clone();

			*parser.parent_node_mut() = func_decl.clone();
			let func_body = parser.statements()?;
			*parser.parent_node_mut() = parent_node;

			if let Node::FuncDecl { body, .. } = &mut func_decl {
				*body = Some(func_body);
			}

			match parser.current_token().token_type() {
				TokenType::Separator if parser.current_token().symbol() == "}" => (),
				_ => return Err(Error::expected(parser.current_token().pos(), "'}'", Some(&format!("'{}'", parser.current_token().symbol()))))
			}
		}
		_ => return Ok(func_decl)
	}

	parser.advance().unwrap_or(());

	Ok(func_decl)
}

pub fn var_def(parser: &mut Parser) -> Result<(Node, Option<Node>), Feedback> {
	let identifier = match parser.current_token().token_type() {
		TokenType::Identifier => Node::identifier(parser.current_token().symbol(), parser.current_token().pos()),
		_ => return Err(Error::expected(parser.current_token().pos(), "identifier", Some(&format!("'{}'", parser.current_token().symbol()))))
	};

	if parser.advance().is_err() {
		return Err(Error::expected(parser.current_token().pos(), "':'", None))
	}

	match parser.current_token().token_type() {
		TokenType::Operator if parser.current_token().symbol() == ":" => (),
		_ => return Err(Error::expected(parser.current_token().pos(), "':'", Some(&format!("'{}'", parser.current_token().symbol()))))
	};

	let var_type = match type_node(parser) {
		Ok(x) => Some(x),
		Err(_) => None
	};

	Ok((identifier, var_type))
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

	if parser.advance().is_err() {
		return Err(Error::expected(parser.current_token().pos(), "identifier", None))
	}

	let (identifier, var_type) = var_def(parser)?;

	let var_type = match var_type {
		Some(x) => x,
		None => return Err(Error::expected(parser.current_token().pos(), "type", Some(&format!("'{}'", parser.current_token().symbol()))))
	};

	let mut var_decl = Node::VarDecl { identifier: Box::new(identifier), var_type: Box::new(var_type), value: Box::new(None), public, global };

	parser.advance().unwrap_or(());

	if !parser.is_more_token() {
		return Ok(var_decl);
	}

	match parser.current_token().token_type() {
		TokenType::Operator if parser.current_token().symbol() == "=" => {
			if parser.advance().is_err() {
				return Err(Error::expected(parser.current_token().pos(), "expression", None))
			}

			if let Node::VarDecl { value, .. } = &mut var_decl {
				*value = Box::new(Some(expr(parser)?));
			}
		}
		_ => ()
	}

	parser.advance().unwrap_or(());

	Ok(var_decl)
}

pub fn assignment(parser: &mut Parser) -> Result<Node, Feedback> {
	let identifier = match parser.current_token().token_type() {
		TokenType::Identifier => Node::identifier(parser.current_token().symbol(), parser.current_token().pos()),
		_ => return Err(Error::expected(parser.current_token().pos(), "'identifier", Some(&format!("'{}'", parser.current_token().symbol()))))
	};

	match parser.current_token().symbol() {
		"(" => return func_call(parser, &identifier),
		"=" | "+=" | "-=" | "*=" | "/=" | "%=" | "<<=" | ">>=" | "&=" | "^=" | "|=" => (),
		_ => return Err(Error::expected(parser.current_token().pos(), "symbol", Some(&format!("'{}'", parser.current_token().symbol()))))
	}

	let operator = Node::operator(parser.current_token().symbol(), parser.current_token().pos());

	if parser.advance().is_err() {
		return Err(Error::expected(parser.current_token().pos(), "expression", None))
	}

	parser.advance().unwrap_or(());

	Ok(Node::Assignment { identifier: Box::new(identifier), operator: Box::new(operator), value: Box::new(expr(parser)?) })
}

/// Creates a `Node::FuncCall`.
pub fn func_call(parser: &mut Parser, identifier: &Node) -> Result<Node, Feedback> {
	let mut params = Vec::new();

	match parser.current_token().token_type() {
		TokenType::Separator if parser.current_token().symbol() == ")" => (),
		_ => {
			loop {
				parser.skip_new_lines();
				params.push(expr(parser)?);

				match parser.current_token().symbol() {
					"," => {
						if parser.advance().is_err() {
							return Err(Error::expected(parser.current_token().pos(), "parameter or ')'", None))
						}
					}
					")" => break,
					_ => return Err(Error::expected(parser.current_token().pos(), "',' or ')'", Some(&format!("'{}'", parser.current_token().symbol()))))
				};
			}
		}
	}

	parser.advance().unwrap_or(());

	Ok(Node::FuncCall { identifier: Box::new(identifier.clone()), params })
}

pub fn if_statement(parser: &mut Parser) -> Result<Node, Feedback> {
	if parser.current_token().symbol() != "(" {
		return Err(Error::expected(parser.current_token().pos(), "(", Some(&format!("'{}'", parser.current_token().symbol()))));
	}

	let condition = expr(parser)?;

	if parser.advance().is_err() {
		return Err(Error::expected(parser.current_token().pos(), "'{'", None));
	}

	parser.skip_new_lines();

	if parser.current_token().symbol() != "{" {
		return Err(Error::expected(parser.current_token().pos(), "{", Some(&format!("'{}'", parser.current_token().symbol()))));
	}

	if parser.advance().is_err() {
		return Err(Error::expected(parser.current_token().pos(), "statements or '}'", None));
	}

	let parent_node = parser.parent_node()
		.clone();

	let mut if_statement = Node::IfStatement { condition: Box::new(condition), body: Vec::new() };

	*parser.parent_node_mut() = if_statement.clone();
	let if_body = parser.statements()?;
	*parser.parent_node_mut() = parent_node;

	if let Node::IfStatement { body, .. } = &mut if_statement {
		*body = if_body;
	}

	match parser.current_token().token_type() {
		TokenType::Separator if parser.current_token().symbol() == "}" => (),
		_ => return Err(Error::expected(parser.current_token().pos(), "'}'", Some(&format!("'{}'", parser.current_token().symbol()))))
	}

	parser.advance().unwrap_or(());

	Ok(if_statement)
}

pub fn loop_statement(parser: &mut Parser) -> Result<Node, Feedback> {
	if parser.current_token().symbol() != "(" {
		return Err(Error::expected(parser.current_token().pos(), "(", Some(&format!("'{}'", parser.current_token().symbol()))));
	}

	let condition = expr(parser)?;

	if parser.advance().is_err() {
		return Err(Error::expected(parser.current_token().pos(), "')'", None));
	}

	parser.skip_new_lines();

	if parser.current_token().symbol() != "{" {
		return Err(Error::expected(parser.current_token().pos(), "{", Some(&format!("'{}'", parser.current_token().symbol()))));
	}

	if parser.advance().is_err() {
		return Err(Error::expected(parser.current_token().pos(), "statements or '}'", None));
	}

	let parent_node = parser.parent_node()
		.clone();

	let mut loop_statement = Node::LoopStatement { condition: Box::new(condition), body: Vec::new() };

	*parser.parent_node_mut() = loop_statement.clone();
	let loop_body = parser.statements()?;
	*parser.parent_node_mut() = parent_node;

	if let Node::LoopStatement { body, .. } = &mut loop_statement {
		*body = loop_body;
	}

	match parser.current_token().token_type() {
		TokenType::Separator if parser.current_token().symbol() == "}" => (),
		_ => return Err(Error::expected(parser.current_token().pos(), "'}'", Some(&format!("'{}'", parser.current_token().symbol()))))
	}

	parser.advance().unwrap_or(());

	Ok(loop_statement)
}