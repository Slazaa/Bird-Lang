use crate::bird::lexer::{Token, TokenType};
use crate::bird::feedback::*;
use crate::bird::parser::statement::*;

/// This enum defines the nodes of the AST.
#[derive(Clone, Debug)]
pub enum Node {
	// ----------
	Literal(String),
	Identifier(String),
	Operator(String),
	// ----------
	Program { body: Vec<Node> },
	// Expressions
	UnaryExpr { operator: Box<Node>, node: Box<Node> },
	BinExpr { operator: Box<Node>, left: Box<Node>, right: Box<Node> },
	// Declarations
	FuncDecl { public: bool, identifier: Box<Node>, params: Vec<(Node, Node)>, return_type: Box<Option<Node>>, body: Option<Vec<Node>> },
	VarDecl { public: bool, global: bool, identifier: Box<Node>, var_type: Box<Node>, value: Box<Option<Node>>, },
	// ----------
	Assignment { identifier: Box<Node>, operator: Box<Node>, value: Box<Node> },
	// ----------
	FuncCall { identifier: Box<Node>, params: Vec<Node> },
	// Statements
	IfStatement { condition: Box<Node>, body: Vec<Node> },
	// Types
	Type { identifier: Box<Node> },
	TypeArray { identifier: Box<Node>, size: Box<Node> },
	TypePtr { identifier: Box<Node>, mutable: bool }
}

/// The `Parser` generates an AST from a `Token` list.
pub struct Parser {
	tokens: Vec<Token>,
	token_index: i32,
	current_token: Option<Token>,
	last_token: Option<Token>,
	parent_node: Node,
	next_pub: Option<Token>
}

impl Parser {
	/// Parse the `Token` list into an AST.
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

		let statements = parser.statements()?;

		if let Node::Program { body } = &mut parser.parent_node {
			*body = statements;
		}

		Ok(parser.parent_node)
	}

	/// Returns an option to a reference to the current token.
	pub fn current_token(&self) -> Option<&Token> {
		self.current_token.as_ref()
	}

	/// Returns an option to a reference to the last token.
	pub fn last_token(&self) -> Option<&Token> {
		self.last_token.as_ref()
	}

	/// Returns a reference the parent node.
	pub fn parent_node(&self) -> &Node {
		&self.parent_node
	}

	/// Returns a mutable reference to the parent node.
	pub fn parent_node_mut(&mut self) -> &mut Node {
		&mut self.parent_node
	}

	/// Returns an option to a reference to the `pub` token.
	/// If it is `Some`, the next token is affected by the `pub` keyword.
	/// Else returns `None`.
	pub fn next_pub(&self) -> Option<&Token> {
		self.next_pub.as_ref()
	}

	/// Returns an option to a mutable reference to next pub.
	pub fn next_pub_mut(&mut self) -> &mut Option<Token> {
		&mut self.next_pub
	}

	/// Advances to the next `Token`.
	/// Sets the current `Token` to `None` if no more `Token`.
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

	/// Advances the current `Token` util it is not a new line `Token`.
	pub fn skip_new_lines(&mut self) {
		while let Some(current_token) = self.current_token.clone() {
			match current_token.token_type() {
				TokenType::Separator if current_token.symbol() == "\n" => self.advance(),
				_ => break
			};
		}
	}

	/// Evaluates the statements.
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

			let statement = statement(self)?;

			if let Some(next_pub) = &self.next_pub {
				return Err(Error::expected(next_pub.pos(), "item", None));
			}

			statements.push(statement);
		}

		Ok(statements)
	}
}