use super::constants::*;
use super::lexer::{Token, TokenType};
use super::feedback::*;

type NodeFunc = fn(&mut Parser) -> Result<Node, Feedback>;

#[derive(Clone, Debug)]
pub enum NodeItem {
	Unknown,
	Literal(String),
	Operator(String),
	Array(String),
	FuncDecl {
		identifier: String,
		params: Vec<(String, String)>,
		return_type: Option<String>
	},
	MembDecl {
		identifier: String,
		param_type: String
	}
}

#[derive(Debug)]
pub struct  Node {
	entry: NodeItem,
	children: Vec<Node>
}

impl Node {
	pub fn new(entry: NodeItem, children: Vec<Node>) -> Self {
		Self {
			entry,
			children
		}
	}

	pub fn children_mut(&mut self) -> &mut Vec<Node> {
		&mut self.children
	}
}

pub struct Parser {
	tokens: Vec<Token>,
	token_index: i32,
	current_token: Option<Token>,
	last_token: Option<Token>,
	parent_node_item: NodeItem
}

impl Parser {
	pub fn parse(tokens: &[Token]) -> Result<Node, Feedback> {
		let mut parser = Self { 
			tokens: tokens.to_vec(),
			token_index: -1,
			current_token: None,
			last_token: None,
			parent_node_item: NodeItem::Unknown
		};

		parser.advance();
		parser.statements("Program")
	}

	fn advance(&mut self) -> Option<Token> {
		self.token_index += 1;

		if self.token_index < self.tokens.len() as i32 {
			self.current_token = Some(self.tokens[self.token_index as usize].clone());
			
			if let Some(current_token) = self.current_token.clone() {
				return Some(current_token);
			}
		}

		self.last_token = self.current_token.clone();
		self.current_token = None;

		None
	}

	fn statements(&mut self, name: &str) -> Result<Node, Feedback> {
		let mut statements = Node::new(NodeItem::Array(name.to_owned()), vec![]);
		self.parent_node_item = statements.entry.clone();

		loop {
			let statement = match self.statement() { 
				Ok(statement) => {
					match &statement.entry {
						NodeItem::Unknown => break,
						_ => statement
					}
				}
				Err(e) => return Err(e)
			};

			statements.children_mut().push(statement);
		}

		Ok(statements)
	}

	fn statement(&mut self) -> Result<Node, Feedback> {
		let current_token = match self.current_token.clone() {
			Some(x) => x,
			None => return Ok(Node::new(NodeItem::Unknown, vec![]))
		};
		
		match *current_token.token_type() {
			TokenType::Keyword => {
				match current_token.symbol() {
					"const" => (),
					_ => ()
				}

				if let NodeItem::Array(name) = &self.parent_node_item {
					if name != "Program" {
						match current_token.symbol() {
							"break" => (),
							"continue" => (),
							"return" => (),
							"var" => (),
							_ => ()
						}
					}
				}
			}
			TokenType::Identifier => {
				todo!();
			}
			_ => ()
		}

		self.control_flow_statement()
	}

	fn control_flow_statement(&mut self) -> Result<Node, Feedback> {
		let current_token = match self.current_token.clone() {
			Some(x) => x,
			None => return Ok(Node::new(NodeItem::Unknown, vec![]))
		};

		match *current_token.token_type() {
			TokenType::Keyword => {
				match current_token.symbol() {
					"func" => return self.func_decl(),
					_ => ()
				}

				if let NodeItem::Array(name) = &self.parent_node_item {
					if name != "Program" {
						match current_token.symbol() {
							"if" => (),
							"else" => (),
							"loop" => (),
							_ => ()
						}
					}
				}
			}
			_ => ()
		}

		return self.expr()
	}

	fn func_decl(&mut self) -> Result<Node, Feedback> {
		let mut current_token = self.advance()
			.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;

		let identifier = match current_token.token_type() {
			TokenType::Identifier => current_token.symbol().to_owned(),
			_ => return Err(Error::expected(current_token.pos(), "'Identifier'", Some(&format!("'{}'", current_token.symbol()))))
		};

		current_token = self.advance()
			.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;

		match current_token.token_type() {
			TokenType::Separator if current_token.symbol() == "(" => (),
			_ => return Err(Error::expected(current_token.pos(), "'('", Some(&format!("'{}'", current_token.symbol()))))
		}

		let mut params = Vec::new();

		current_token = self.advance()
			.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;

		match current_token.token_type() {
			TokenType::Separator if current_token.symbol() == ")" => (),
			_ => {
				loop {
					let param_entry = match self.memb_decl() {
						Ok(param) => param.entry,
						Err(e) => return Err(e) 
					};
		
					if let NodeItem::MembDecl { identifier, param_type } = param_entry {
						params.push((identifier, param_type));
					}
		
					current_token = self.advance()
						.ok_or_else(|| Error::expected(current_token.pos(), "',' or ')'", None))?;
		
					match current_token.symbol() {
						"," => self.advance(),
						")" => break,
						_ => return Err(Error::expected(current_token.pos(), "',' or ')'", Some(&format!("'{}'", current_token.symbol()))))
					};
				}
			}
		}

		current_token = self.advance()
			.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;

		let return_type = match current_token.token_type() {
			TokenType::Operator if current_token.symbol() == "->" => {
				current_token = self.advance()
					.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;

				self.advance();

				match current_token.token_type() {
					TokenType::Identifier => Some(current_token.symbol().to_owned()),
					_ => return Err(Error::expected(current_token.pos(), "'Identifier'", Some(&format!("'{}'", current_token.symbol()))))
				}
			}
			_ => None
		};

		current_token = match self.current_token.clone() {
			Some(x) => x,
			None => return Ok(Node::new(NodeItem::Unknown, vec![]))
		};

		match current_token.token_type() {
			TokenType::Separator if current_token.symbol() == "{" => (),
			_ => return Err(Error::expected(current_token.pos(), "'{'", Some(&format!("'{}'", current_token.symbol()))))
		}

		let mut func_decl = Node::new(NodeItem::FuncDecl { identifier, params, return_type }, vec![]);

		current_token = self.advance()
			.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;

		match current_token.token_type() {
			TokenType::Separator if current_token.symbol() == "}" => (),
			_ => {
				let func_body = match self.statements("Body") {
					Ok(x) => x,
					Err(e) => return Err(e)
				};
		
				func_decl.children_mut()
					.push(func_body);
		
				current_token = self.advance()
					.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;
		
				match current_token.token_type() {
					TokenType::Separator if current_token.symbol() == "}" => (),
					_ => return Err(Error::expected(current_token.pos(), "'}'", Some(&format!("'{}'", current_token.symbol()))))
				}
			}
		}

		self.advance();

		Ok(func_decl)
	}

	fn memb_decl(&mut self) -> Result<Node, Feedback> {
		let mut current_token = self.current_token.clone()
			.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;

		let identifier = match current_token.token_type() {
			TokenType::Identifier => current_token.symbol().to_owned(),
			_ => return Err(Error::expected(current_token.pos(), "'Identifier'", Some(&format!("'{}'", current_token.symbol()))))
		};

		current_token = self.advance()
			.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;

		match current_token.token_type() {
			TokenType::Operator if current_token.symbol() == ":" => (),
			_ => return Err(Error::expected(current_token.pos(), "':'", Some(&format!("'{}'", current_token.symbol()))))
		}

		current_token = self.advance()
			.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;

		let param_type = match current_token.token_type() {
			TokenType::Identifier => current_token.symbol().to_owned(),
			_ => return Err(Error::expected(current_token.pos(), "':'", Some(&format!("'{}'", current_token.symbol()))))
		};

		Ok(Node::new(NodeItem::MembDecl { identifier, param_type }, vec![]))
	}
	
	fn expr(&mut self) -> Result<Node, Feedback> {
		self.binary_op(Self::term, None, vec!["+", "-"])
	}

	fn term(&mut self) -> Result<Node, Feedback> {
		self.binary_op(Self::factor, None, vec!["*", "/", "%"])
	}

	fn factor(&mut self) -> Result<Node, Feedback> {
		let mut current_token = self.current_token.clone()
			.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;

		if *current_token.token_type() == TokenType::Literal {
			self.advance();
			return Ok(Node::new(NodeItem::Literal(current_token.symbol().to_owned()), vec![]));
		} else if *current_token.token_type() == TokenType::Operator && "+-".contains(current_token.symbol()) {
			self.advance();
			let factor = self.factor()?;
			return Ok(Node::new(NodeItem::Operator(current_token.symbol().to_owned()), vec![factor]));
		} else if *current_token.token_type() == TokenType::Separator && "(".contains(current_token.symbol()) {
			self.advance();
			let expr = self.expr()?;

			current_token = self.current_token.clone()
				.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;

			if *current_token.token_type() == TokenType::Separator && ")".contains(current_token.symbol()) {
				self.advance();
				return Ok(expr);
			} else {
				current_token = self.current_token.clone()
					.ok_or_else(|| Error::invalid_syntax(None, "Invalid syntax"))?;

				return Err(Error::invalid_syntax(Some((current_token.pos_start(), current_token.pos_end())), "Expected ')'"));
			}
		}

		if let Some(last_token) = self.last_token.clone() {
			let mut pos_start = last_token.pos_start().clone();
			let mut pos_end = last_token.pos_end().clone();

			*pos_start.colomn_mut() += 2;
			*pos_end.colomn_mut() += 2;

			return Err(Error::expected((&pos_start, &pos_end), "Expected number", None));
		}

		Err(Error::expected(current_token.pos(), "number", Some(&format!("'{}'", current_token.symbol()))))
	}

	fn binary_op(
		&mut self,
		first_func: NodeFunc,
		second_func: Option<NodeFunc>,
		ops: Vec<&str>
	) -> Result<Node, Feedback> {
		let mut func = first_func;

		if let Some(second_func) = second_func {
			func = second_func;
		}

		let second_func = func;
		let mut left = first_func(self)?;

		if let Some(token) = self.current_token.clone() {
			if *token.token_type() != TokenType::Operator && *token.token_type() != TokenType::Separator {
				return Err(Error::expected((token.pos_start(), token.pos_end()), "operator", Some(&format!("'{}'", token.symbol()))));
			}
			
			while let Some(token) = self.current_token.clone() {
				if !ops.contains(&token.symbol()) {
					break;
				}

				if !OPERATORS.contains(&token.symbol()) {
					return Err(Error::invalid_syntax(Some((token.pos_start(), token.pos_end())), "Invalid operator"))
				}

				let token_operator = NodeItem::Operator(token.symbol().to_owned());

				self.advance();

				let old_left = left;
				let right = second_func(self)?;

				left = Node::new(token_operator, vec![old_left, right]);
			}
		}

		Ok(left)
	}
}