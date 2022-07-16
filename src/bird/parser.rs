use super::lexer::{Token, TokenType};
use super::feedback::*;

#[derive(Clone, Debug)]
pub enum NodeItem {
	Literal(String),
	Operator(String)
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
	current_token: Option<Token>,
	last_token: Option<Token>
}

impl Parser {
	pub fn parse(tokens: Vec<Token>) -> Result<Node, Feedback> {
		let mut parser = Self { 
			tokens,
			token_index: -1,
			current_token: None,
			last_token: None
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

		self.last_token = self.current_token.clone();
		self.current_token = None;
	}

	fn factor(&mut self) -> Result<Node, Feedback> {
		let current_token = match self.current_token.clone() {
			Some(x) => x,
			None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
		};

		if *current_token.token_type() == TokenType::Literal {
			self.advance();
			return Ok(Node::new(NodeItem::Literal(current_token.symbol().to_owned())));
		}

		if let Some(last_token) = self.last_token.clone() {
			let mut pos_start = last_token.pos_start().clone();
			let mut pos_end = last_token.pos_end().clone();

			*pos_start.colomn_mut() += 2;
			*pos_end.colomn_mut() += 2;

			return Err(Error::invalid_syntax(Some((&pos_start, &pos_end)), "Expected number"));
		}

		Err(Error::invalid_syntax(Some((current_token.pos_start(), current_token.pos_end())), &format!("Expected number found '{}'", current_token.symbol())))
	}

	fn term(&mut self) -> Result<Node, Feedback> {
		self.binary_op(Self::factor, "*/%")
	}

	fn expr(&mut self) -> Result<Node, Feedback> {
		self.binary_op(Self::term, "+-")
	}

	fn binary_op(&mut self, func: fn(&mut Self) -> Result<Node, Feedback>, operators: &str) -> Result<Node, Feedback> {
		let mut left = func(self)?;

		if let Some(token) = self.current_token.clone() {
			if *token.token_type() != TokenType::Operator {
				return Err(Error::invalid_syntax(Some((token.pos_start(), token.pos_end())), "Expected operator"));
			}

			while let Some(token) = self.current_token.clone() {
				if !operators.contains(token.symbol()) {
					break;
				}

				if !"+-*/".contains(token.symbol()) {
					return Err(Error::invalid_syntax(Some((token.pos_start(), token.pos_end())), "Invalid operator"))
				}

				let token_operator = NodeItem::Operator(token.symbol().to_owned());

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