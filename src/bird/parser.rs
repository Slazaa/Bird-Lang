use super::lexer::{Token, TokenType};
use super::error::Error;

#[derive(Clone, Copy, Debug)]
pub enum Operator {
	Add,
	Sub,
	Mult,
	Div
}

#[derive(Clone, Debug)]
pub enum NodeItem {
	Literal(String),
	Operator(Operator)
}

#[derive(Debug)]
pub struct  Node {
	entry: NodeItem,
	children: Vec<Node>
}

impl Node {
	pub fn new(entry: NodeItem) -> Self {
		Self {
			entry,
			children: Vec::new()
		}
	}

	pub fn children_mut(&mut self) -> &mut Vec<Node> {
		&mut self.children
	}
}

pub struct Parser {
	tokens: Vec<Token>,
	token_index: i32,
	current_token: Option<Token>
}

impl Parser {
	pub fn parse(tokens: Vec<Token>) -> Result<Node, Error> {
		let mut parser = Self { 
			tokens,
			token_index: -1,
			current_token: None
		};

		parser.advance();
		parser.expr()
	}

	fn advance(&mut self) {
		self.token_index += 1;

		if self.token_index < self.tokens.len() as i32 {
			self.current_token = Some(self.tokens[self.token_index as usize].clone());
			return;
		}

		self.current_token = None;
	}

	fn factor(&mut self) -> Result<Node, Error> {
		let token = match self.current_token.clone() {
			Some(x) => x,
			None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
		};

		if *token.token_type() == TokenType::Literal {
			self.advance();
			return Ok(Node::new(NodeItem::Literal(token.symbol().to_owned())));
		}

		Err(Error::invalid_syntax(Some((token.pos_start().clone(), token.pos_end().clone())), "Expected number"))
	}

	fn term(&mut self) -> Result<Node, Error> {
		self.binary_op(Self::factor, "*/")
	}

	fn expr(&mut self) -> Result<Node, Error> {
		self.binary_op(Self::term, "+-")
	}

	fn binary_op(&mut self, func: fn(&mut Self) -> Result<Node, Error>, operators: &str) -> Result<Node, Error> {
		let mut left = func(self)?;

		if let Some(token) = self.current_token.clone() {
			if *token.token_type() != TokenType::Operator {
				return Err(Error::invalid_syntax(Some((token.pos_start().clone(), token.pos_end().clone())), "Expected operator"));
			}

			loop {
				let token = match self.current_token.clone() {
					Some(x) => x,
					None => break
				};

				if !operators.contains(token.symbol()) {
					break;
				}

				let token_operator = NodeItem::Operator(match token.symbol() {
					"+" => Operator::Add,
					"-" => Operator::Sub,
					"*" => Operator::Mult,
					"/" => Operator::Div,
					_ => return Err(Error::invalid_syntax(Some((token.pos_start().clone(), token.pos_end().clone())), "Invalid operator"))
				});

				self.advance();

				let old_left = left;
				let right = func(self)?;

				left = Node::new(token_operator);

				left.children_mut().push(old_left);
				left.children_mut().push(right);
			}
		}

		Ok(left)
	}
}