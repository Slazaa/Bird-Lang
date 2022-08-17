use crate::bird::lexer::{Token, TokenType, Position};
use crate::bird::feedback::*;

/// This enum defines the nodes of the AST.
#[derive(Clone, Debug)]
pub enum Node {
	Undefined,
	// ----------
	Literal(String, (Position, Position)),
	Identifier(String, (Position, Position)),
	Operator(String, (Position, Position)),
	// ----------
	Program { body: Vec<Node> },
	// Expressions
	UnaryExpr { operator: Box<Node>, node: Box<Node> },
	BinExpr { operator: Box<Node>, left: Box<Node>, right: Box<Node> },
	// Declarations
	FuncDecl { public: bool, identifier: Box<Node>, params: Vec<(Node, Node)>, return_type: Box<Option<Node>>, body: Option<Vec<Node>> },
	VarDecl { public: bool, global: bool, identifier: Box<Node>, var_type: Box<Node>, value: Box<Option<Node>> },
	// ----------
	Assignment { identifier: Box<Node>, operator: Box<Node>, value: Box<Node> },
	// ----------
	FuncCall { identifier: Box<Node>, params: Vec<Node> },
	// Statements
	IfStatement { condition: Box<Node>, body: Vec<Node> },
	LoopStatement { condition: Box<Node>, body: Vec<Node> },
	// Types
	Type { identifier: Box<Node> },
	TypeArray { identifier: Box<Node>, size: Box<Node> },
	TypePtr { identifier: Box<Node>, mutable: bool }
}

impl Node {
	pub fn literal(value: &str, pos: (&Position, &Position)) -> Self {
		let pos = (pos.0.clone(), pos.1.clone());
		Self::Literal(value.to_owned(), pos)
	}

	pub fn identifier(value: &str, pos: (&Position, &Position)) -> Self {
		let pos = (pos.0.clone(), pos.1.clone());
		Self::Identifier(value.to_owned(), pos)
	}

	pub fn operator(value: &str, pos: (&Position, &Position)) -> Self {
		let pos = (pos.0.clone(), pos.1.clone());
		Self::Operator(value.to_owned(), pos)
	}
}

/// The `Parser` generates an AST from a `Token` list.
pub struct Parser {
	tokens: Vec<Token>,
	token_index: usize,
	current_token: Token,
	parent_node: Node,
	next_pub: Option<Token>
}

impl Parser {
	/// Parse the `Token` list into an AST.
	pub fn parse(tokens: &[Token]) -> Result<Node, Feedback> {
		let current_token = match tokens.first() {
			Some(x) => x.clone(),
			None => return Ok(Node::Program { body: vec![] })
		};

		let mut parser = Self { 
			tokens: tokens.to_vec(),
			token_index: 0,
			current_token,
			parent_node: Node::Program { body: Vec::new() },
			next_pub: None
		};

		let block = parser.block()?;

		if let Node::Program { body } = &mut parser.parent_node {
			*body = block;
		}

		Ok(parser.parent_node)
	}

	fn is_more_token(&self) -> bool {
		self.token_index < self.tokens.len()
	}

	/// Returns reference to the current token.
	fn current_token(&self) -> &Token {
		&self.current_token
	}

	/// Returns a reference the parent node.
	fn parent_node(&self) -> &Node {
		&self.parent_node
	}

	/// Returns a mutable reference to the parent node.
	fn parent_node_mut(&mut self) -> &mut Node {
		&mut self.parent_node
	}

	/// Returns an option to a reference to the `pub` token.
	/// If it is `Some`, the next token is affected by the `pub` keyword.
	/// Else returns `None`.
	fn next_pub(&self) -> Option<&Token> {
		self.next_pub.as_ref()
	}

	/// Returns an option to a mutable reference to next pub.
	fn next_pub_mut(&mut self) -> &mut Option<Token> {
		&mut self.next_pub
	}

	/// Advances to the next `Token`.
	/// Returns a reference to the current `Node`.
	fn advance(&mut self) -> Result<(), ()> {
		self.token_index += 1;
		
		if self.token_index < self.tokens.len() {
			self.current_token = self.tokens[self.token_index].clone();
			return Ok(());
		}

		Err(())
	}

	/// Advances the current `Token` util it is not a new line `Token`.
	fn skip_new_lines(&mut self) {
		if !self.is_more_token() {
			return;
		}

		loop {
			match self.current_token().token_type() {
				TokenType::Separator if self.current_token().symbol() == "\n" => (),
				_ => break
			};

			if self.advance().is_err() {
				break;
			}
		}
	}

	fn check_scope(&self) -> bool {
		match self.parent_node() {
			Node::Program { .. } => {
				match self.current_token().token_type() {
					TokenType::Keyword => matches!(
						self.current_token().symbol(),
						"const" | "func" | "var"
					),
					_ => false
				}
			}
			_ => {
				match self.current_token().token_type() {
					TokenType::Literal => true,
					TokenType::Identifier => true,
					TokenType::Separator if self.current_token().symbol() == "{" => true,
					TokenType::Keyword => matches!(
						self.current_token().symbol(),
						"break" | "continue" | "const" |
						"else"  | "func"     | "if"    |
						"loop"  | "return"   | "var"
					),
					_ => false
				}
			}
		}
	}

	fn block(&mut self) -> Result<Vec<Node>, Feedback> {
		let mut statements = Vec::new();

		loop {
			self.skip_new_lines();

			if !self.is_more_token() {
				break;
			}

			match &self.parent_node {
				Node::Program { .. } => (),
				_ if self.current_token().symbol() == "}" => break,
				_ => ()
			}

			match self.current_token().token_type() {
				TokenType::Keyword if self.current_token().symbol() == "pub" => {
					self.next_pub = Some(self.current_token().clone());
					
					if self.advance().is_err() {
						return Err(Error::expected(self.current_token().pos(), "'Item'", None))
					}

					continue;
				}
				_ => ()
			}

			let statement = self.eval()?;

			if let Some(next_pub) = &self.next_pub {
				return Err(Error::expected(next_pub.pos(), "item", None));
			}

			statements.push(statement);
		}

		Ok(statements)
	}

	fn eval(&mut self) -> Result<Node, Feedback> {
		if let Some(next_pub) = self.next_pub() {
			match self.current_token().symbol() {
				"func" => (),
				_ => return Err(Error::unexpected(next_pub.pos(), &format!("'{}'", self.current_token().symbol())))
			}
		}

		if !self.check_scope() {
			return Err(Error::unexpected(self.current_token().pos(), &format!("'{}'", self.current_token().symbol())));
		}
		
		match self.current_token().token_type() {
			TokenType::Literal => self.bin_expr(),
			TokenType::Identifier => self.assignment(),
			TokenType::Keyword => {
				match self.current_token().symbol() {
					"break" => todo!(),
					"const" => todo!(),
					"continue" => todo!(),
					"func" => self.func_decl(),
					"if" => self.if_statement(),
					"loop" => self.loop_statement(),
					"return" => todo!(),
					"var" => self.var_decl(),
					_ => todo!()
				}
			}
			_ => todo!()
		}
	}

	fn type_node(&mut self) -> Result<Node, Feedback> {
		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "type", None))
		}
	
		match self.current_token().token_type() {
			TokenType::Identifier => Ok(Node::Type { identifier: Box::new(Node::identifier(self.current_token().symbol(), self.current_token().pos())) }),
			_ => self.type_ptr_node()
		}
	}

	fn type_ptr_node(&mut self) -> Result<Node, Feedback> {
		match self.current_token().token_type() {
			TokenType::Operator if self.current_token().symbol() == "*" => {
				if self.advance().is_err() {
					return Err(Error::expected(self.current_token().pos(), "'mut' or type", None))
				}
	
				let mutable = match self.current_token().token_type() {
					TokenType::Keyword if self.current_token().symbol() == "mut" => {
						if self.advance().is_err() {
							return Err(Error::expected(self.current_token().pos(), "type", None))
						}
	
						true
					},
					_ => false
				};
	
				let identifier = match self.current_token().token_type() {
					TokenType::Identifier => Node::identifier(self.current_token().symbol(), self.current_token().pos()),
					_ => return Err(Error::expected(self.current_token().pos(), "type", Some(&format!("'{}'", self.current_token().symbol()))))
				};
	
				Ok(Node::TypePtr { identifier: Box::new(identifier), mutable })
			}
			_ => self.type_array_node()
		}
	}

	fn type_array_node(&mut self) -> Result<Node, Feedback> {
		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == "[" => {
				let identifier = match self.current_token().token_type() {
					TokenType::Identifier => Node::identifier(self.current_token().symbol(), self.current_token().pos()),
					_ => return Err(Error::expected(self.current_token().pos(), "type", Some(&format!("'{}'", self.current_token().symbol()))))
				};
	
				match self.current_token().token_type() {
					TokenType::Separator if self.current_token().symbol() == "," => (),
					_ => return Err(Error::expected(self.current_token().pos(), "','", Some(&format!("'{}'", self.current_token().symbol()))))
				}
	
				let size = match self.current_token().token_type() {
					TokenType::Literal => Node::literal(self.current_token().symbol(), self.current_token().pos()),
					_ => return Err(Error::expected(self.current_token().pos(), "literal", Some(&format!("'{}'", self.current_token().symbol()))))
				};
	
				match self.current_token().token_type() {
					TokenType::Separator if self.current_token().symbol() == "]" => (),
					_ => return Err(Error::expected(self.current_token().pos(), "']'", Some(&format!("'{}'", self.current_token().symbol()))))
				}
	
				Ok(Node::TypeArray { identifier: Box::new(identifier), size: Box::new(size) })
			}
			_ => Err(Error::expected(self.current_token().pos(), "type", Some(&format!("'{}'", self.current_token().symbol()))))
		}
	}

	fn var_def(&mut self) -> Result<(Node, Node), Feedback> {
		let identifier = match self.current_token().token_type() {
			TokenType::Identifier => Node::identifier(self.current_token().symbol(), self.current_token().pos()),
			_ => return Err(Error::expected(self.current_token().pos(), "identifier", Some(&format!("'{}'", self.current_token().symbol()))))
		};
	
		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "':'", None))
		}
	
		match self.current_token().token_type() {
			TokenType::Operator if self.current_token().symbol() == ":" => (),
			_ => return Err(Error::expected(self.current_token().pos(), "':'", Some(&format!("'{}'", self.current_token().symbol()))))
		};
	
		Ok((identifier, self.type_node()?))
	}

	fn func_decl(&mut self) -> Result<Node, Feedback> {
		let public = match self.next_pub() {
			Some(_) => {
				*self.next_pub_mut() = None;
				true
			},
			None => false
		};
	
		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "identifier", None))
		}
	
		let identifier = match self.current_token().token_type() {
			TokenType::Identifier => {
				let identifier = self.current_token().symbol().to_owned();
				Node::identifier(&identifier, self.current_token().pos())
			}
			_ => return Err(Error::expected(self.current_token().pos(), "identifier", Some(&format!("'{}'", self.current_token().symbol()))))
		};
	
		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "'('", None))
		}
	
		self.skip_new_lines();
	
		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == "(" => (),
			_ => return Err(Error::expected(self.current_token().pos(), "'('", Some(&format!("'{}'", self.current_token().symbol()))))
		}
	
		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "'('", None))
		}
	
		self.skip_new_lines();
	
		let mut params = Vec::new();
	
		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == ")" => (),
			_ => {
				loop {
					self.skip_new_lines();

					params.push(self.var_def()?);
		
					if self.advance().is_err() {
						return Err(Error::expected(self.current_token().pos(), "',' or ')'", None))
					}
	
					self.skip_new_lines();
	
					match self.current_token().symbol() {
						"," => {
							if self.advance().is_err() {
								return Err(Error::expected(self.current_token().pos(), "identifier", None))
							}		
						},
						")" => break,
						_ => return Err(Error::expected(self.current_token().pos(), "',' or ')'", Some(&format!("'{}'", self.current_token().symbol()))))
					};
				}
			}
		}
	
		let mut func_decl = Node::FuncDecl { public, identifier: Box::new(identifier), params, return_type: Box::new(None), body: None };
	
		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "'->' or '{'", None))
		}
	
		match self.current_token().token_type() {
			TokenType::Operator if self.current_token().symbol() == "->" => {
				if let Node::FuncDecl { return_type, .. } = &mut func_decl {
					*return_type = Box::new(Some(self.type_node()?));
				}
	
				if self.advance().is_err() {
					return Err(Error::expected(self.current_token().pos(), "'->' or '{'", None))
				}
			}
			_ => ()
		};
	
		self.skip_new_lines();
	
		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == "{" => {
				if self.advance().is_err() {
					return Err(Error::expected(self.current_token().pos(), "statement or '}'", None))
				}
	
				let parent_node = self.parent_node()
					.clone();
	
				*self.parent_node_mut() = func_decl.clone();
				let func_body = self.block()?;
				*self.parent_node_mut() = parent_node;
	
				if let Node::FuncDecl { body, .. } = &mut func_decl {
					*body = Some(func_body);
				}
	
				match self.current_token().token_type() {
					TokenType::Separator if self.current_token().symbol() == "}" => (),
					_ => return Err(Error::expected(self.current_token().pos(), "'}'", Some(&format!("'{}'", self.current_token().symbol()))))
				}
			}
			_ => return Ok(func_decl)
		}
	
		self.advance().unwrap_or(());
	
		Ok(func_decl)
	}

	fn var_decl(&mut self) -> Result<Node, Feedback> {
		let mut public = false;
		let mut global = false;
	
		if let Node::Program { .. } = self.parent_node() {
			if self.next_pub().is_some() {
				*self.next_pub_mut() = None;
				public = true;
			}
	
			global = true;
		}
	
		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "identifier", None))
		}
	
		let (identifier, var_type) = self.var_def()?;
		let mut var_decl = Node::VarDecl { identifier: Box::new(identifier), var_type: Box::new(var_type), value: Box::new(None), public, global };
	
		self.advance().unwrap_or(());
	
		if !self.is_more_token() {
			return Ok(var_decl);
		}
	
		match self.current_token().token_type() {
			TokenType::Operator if self.current_token().symbol() == "=" => {
				if self.advance().is_err() {
					return Err(Error::expected(self.current_token().pos(), "expression", None))
				}
	
				if let Node::VarDecl { value, .. } = &mut var_decl {
					*value = Box::new(Some(self.eval()?));
				}
			}
			_ => ()
		}
	
		self.advance().unwrap_or(());
	
		Ok(var_decl)
	}

	fn assignment(&mut self) -> Result<Node, Feedback> {
		let identifier = match self.current_token().token_type() {
			TokenType::Identifier => Node::identifier(self.current_token().symbol(), self.current_token().pos()),
			_ => return Err(Error::expected(self.current_token().pos(), "'identifier", Some(&format!("'{}'", self.current_token().symbol()))))
		};
	
		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "operator", None))
		}
	
		match self.current_token().symbol() {
			"=" | "+=" | "-=" | "*=" | "/=" | "%=" | "<<=" | ">>=" | "&=" | "^=" | "|=" => (),
			_ => return self.func_call(&identifier)
		}
	
		let operator = Node::operator(self.current_token().symbol(), self.current_token().pos());
	
		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "expression", None))
		}
	
		Ok(Node::Assignment { identifier: Box::new(identifier), operator: Box::new(operator), value: Box::new(self.eval()?) })
	}

	fn func_call(&mut self, identifier: &Node) -> Result<Node, Feedback> {
		match self.current_token().symbol() {
			"(" => (),
			_ => return Err(Error::expected(self.current_token().pos(), "assignement or function call", None))
		}

		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "parameters or ')'", None))
		}
	
		let mut params = Vec::new();
	
		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == ")" => (),
			_ => {
				loop {
					self.skip_new_lines();
					params.push(self.eval()?);
	
					match self.current_token().symbol() {
						"," => {
							if self.advance().is_err() {
								return Err(Error::expected(self.current_token().pos(), "parameter or ')'", None))
							}
						}
						")" => break,
						_ => return Err(Error::expected(self.current_token().pos(), "',' or ')'", Some(&format!("'{}'", self.current_token().symbol()))))
					};
				}
			}
		}
	
		self.advance().unwrap_or(());
	
		Ok(Node::FuncCall { identifier: Box::new(identifier.clone()), params })
	}

	fn control_flow(&mut self, node: &mut Node) -> Result<(), Feedback> {
		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "'('", None));
		}

		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == "(" => (),
			_ => return Err(Error::expected(self.current_token().pos(), "'('", Some(self.current_token().symbol())))
		}

		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "'('", None));
		}

		match node {
			Node::IfStatement { condition, .. } => *condition = Box::new(self.eval()?),
			Node::LoopStatement { condition, .. } => *condition = Box::new(self.eval()?),
			_ => return Err(Error::unspecified("Invalid node"))
		}
	
		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "'{'", None));
		}
	
		self.skip_new_lines();
	
		if self.current_token().symbol() != "{" {
			return Err(Error::expected(self.current_token().pos(), "{", Some(&format!("'{}'", self.current_token().symbol()))));
		}
	
		if self.advance().is_err() {
			return Err(Error::expected(self.current_token().pos(), "statements or '}'", None));
		}
	
		let parent_node = self.parent_node()
			.clone();
	
		*self.parent_node_mut() = node.clone();
		let block = self.block()?;
		*self.parent_node_mut() = parent_node;

		match node {
			Node::IfStatement { body, .. } => *body = block,
			Node::LoopStatement { body, .. } => *body = block,
			_ => return Err(Error::unspecified("Invalid node"))
		}
	
		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == "}" => (),
			_ => return Err(Error::expected(self.current_token().pos(), "'}'", Some(&format!("'{}'", self.current_token().symbol()))))
		}
	
		self.advance().unwrap_or(());
	
		Ok(())
	}

	fn if_statement(&mut self) -> Result<Node, Feedback> {
		let mut node = Node::IfStatement { condition: Box::new(Node::Undefined), body: vec![] };

		self.control_flow(&mut node)?;

		Ok(node)
	}

	fn loop_statement(&mut self) -> Result<Node, Feedback> {
		let mut node = Node::LoopStatement { condition: Box::new(Node::Undefined), body: vec![] };

		self.control_flow(&mut node)?;

		Ok(node)
	}

	fn bin_expr(&mut self) -> Result<Node, Feedback> {
		let mut current_token = self.current_token().clone();
		let left = self.literal(&current_token)?;

		let operator = match self.current_token().token_type() {
			TokenType::Operator => {
				current_token = self.current_token().clone();
				self.operator(&current_token)?
			}
			_ => return Ok(left)
		};
		
		Ok(Node::BinExpr { operator: Box::new(operator), left: Box::new(left), right: Box::new(self.eval()?) })
	}

	fn literal(&mut self, token: &Token) -> Result<Node, Feedback> {
		let res = Node::literal(token.symbol(), token.pos());
		self.advance().unwrap_or(());
		Ok(res)
	}

	fn operator(&mut self, token: &Token) -> Result<Node, Feedback> {
		let res = Node::operator(token.symbol(), token.pos());
		self.advance().unwrap_or(());
		Ok(res)
	}
}