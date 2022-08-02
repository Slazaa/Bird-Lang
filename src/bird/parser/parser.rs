use crate::bird::lexer::{Token, TokenType};
use crate::bird::feedback::*;
use crate::bird::parser::statement::*;

pub type NodeFunc = fn(&mut Parser) -> Result<Node, Feedback>;

#[derive(Clone, Debug)]
pub enum NodeItem {
	Unknown,
	Literal(String),
	Operator(String),
	Array(String),
	FuncDecl {
		identifier: String,
		params: Vec<(String, String)>,
		return_type: Option<String>,
		public: bool
	},
	MembDecl {
		identifier: String,
		param_type: String
	},
	VarDecl {
		identifier: String,
		var_type: String,
		public: bool,
		global: bool
	},
	FuncCall {
		identifier: String,
		params: Vec<String>
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

	pub fn entry(&self) -> &NodeItem {
		&self.entry
	}

	pub fn children(&self) -> &Vec<Node> {
		&self.children
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
	parent_node_item: NodeItem,
	next_pub: Option<Token>
}

impl Parser {
	pub fn parse(tokens: &[Token]) -> Result<Node, Feedback> {
		let mut parser = Self { 
			tokens: tokens.to_vec(),
			token_index: -1,
			current_token: None,
			last_token: None,
			parent_node_item: NodeItem::Unknown,
			next_pub: None
		};

		parser.advance();
		parser.statements("Program")
	}

	pub fn current_token(&self) -> Option<&Token> {
		self.current_token.as_ref()
	}

	pub fn last_token(&self) -> Option<&Token> {
		self.last_token.as_ref()
	}

	pub fn parent_node_item(&self) -> &NodeItem {
		&self.parent_node_item
	}

	pub fn parent_node_item_mut(&mut self) -> &mut NodeItem {
		&mut self.parent_node_item
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

	pub fn statements(&mut self, name: &str) -> Result<Node, Feedback> {
		let mut statements = Node::new(NodeItem::Array(name.to_owned()), vec![]);
		self.parent_node_item = statements.entry.clone();

		loop {
			self.skip_new_lines();

			let current_token = match self.current_token.clone() {
				Some(x) => x,
				None => break
			};

			match &self.parent_node_item {
				NodeItem::Array(name) if name != "Program" && current_token.symbol() == "}" => break,
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

			statements.children_mut()
				.push(statement);
		}

		Ok(statements)
	}
}