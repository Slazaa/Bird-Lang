use crate::bird::lexer::{Token, TokenType};
use crate::bird::feedback::*;
use crate::bird::parser::statement::*;

/// This enum defines the nodes of the AST.
#[derive(Clone, Debug)]
pub enum Node {
	Literal(String),
	Identifier(String),
	Program {
		body: Vec<Node>
	},
	UnaryExpr {
		operator: String,
		node: Box<Node>
	},
	BinExpr {
		operator: String,
		left: Box<Node>,
		right: Box<Node>
	},
	FuncDecl {
		public: bool,
		identifier: String,
		params: Vec<Node>,
		return_type: Option<String>,
		body: Option<Vec<Node>>
	},
	MembDecl {
		identifier: String,
		param_type: String
	},
	VarDecl {
		public: bool,
		global: bool,
		identifier: String,
		var_type: String,
		value: Option<Box<Node>>,
	},
	Assignment {
		identifier: String,
		operator: String,
		value: Box<Node>
	},
	FuncCall {
		identifier: String,
		params: Vec<Node>
	}
}

pub struct Parser {
	tokens: Vec<Token>,
	token_index: i32,
	current_token: Option<Token>,
	last_token: Option<Token>,
	parent_node: Node,
	next_pub: Option<Token>
}

impl Parser {
	pub fn parse(tokens: &[Token]) -> Result<Node, Feedback> {
		let mut parser = Self { 
			tokens: tokens.to_vec(),
			token_index: -1,
			current_token: None,
			last_token: None,
			parent_node: Node::Program { body: Vec::new() },
			next_pub: None
		};

		parser.advance();

		let statements = match parser.statements() {
			Ok(x) => x,
			Err(e) => return Err(e)
		};

		if let Node::Program { body } = &mut parser.parent_node {
			*body = statements;
		}

		Ok(parser.parent_node)
	}

	pub fn current_token(&self) -> Option<&Token> {
		self.current_token.as_ref()
	}

	pub fn last_token(&self) -> Option<&Token> {
		self.last_token.as_ref()
	}

	pub fn parent_node(&self) -> &Node {
		&self.parent_node
	}

	pub fn parent_node_mut(&mut self) -> &mut Node {
		&mut self.parent_node
	}

	pub fn next_pub(&self) -> Option<&Token> {
		self.next_pub.as_ref()
	}

	pub fn next_pub_mut(&mut self) -> &mut Option<Token> {
		&mut self.next_pub
	}

	pub fn advance(&mut self) -> Option<&Token> {
		self.token_index += 1;
		self.last_token = self.current_token.clone();

		if self.token_index < self.tokens.len() as i32 {
			self.current_token = Some(self.tokens[self.token_index as usize].clone());

			if self.current_token.is_some() {
				return self.current_token();
			}
		}

		self.current_token = None;

		None
	}

	pub fn skip_new_lines(&mut self) {
		while let Some(current_token) = self.current_token.clone() {
			match current_token.token_type() {
				TokenType::Separator if current_token.symbol() == "\n" => self.advance(),
				_ => break
			};
		}
	}

	pub fn statements(&mut self) -> Result<Vec<Node>, Feedback> {
		let mut statements = Vec::new();

		loop {
			self.skip_new_lines();

			let current_token = match self.current_token.clone() {
				Some(x) => x,
				None => break
			};

			match &self.parent_node {
				Node::Program { .. } => (),
				_ if current_token.symbol() == "}" => break,
				_ => ()
			}

			match current_token.token_type() {
				TokenType::Keyword if current_token.symbol() == "pub" => {
					self.next_pub = Some(current_token.clone());
					self.advance();
					continue;
				}
				_ => ()
			}

			let statement = match statement(self) { 
				Ok(x) => x,
				Err(e) => return Err(e)
			};

			if let Some(next_pub) = &self.next_pub {
				return Err(Error::expected(next_pub.pos(), "item", None));
			}

			statements.push(statement);
		}

		Ok(statements)
	}
}