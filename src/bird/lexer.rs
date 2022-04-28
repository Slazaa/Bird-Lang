use super::error::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TokenType {
	Operator,
	Literal
}

pub struct Token {
	token_type: TokenType,
	symbol: String
}

impl Token {
	pub fn new(token_type: TokenType, symbol: &str) -> Self {
		Self {
			token_type,
			symbol: symbol.to_owned()
		}
	}

	pub fn token_type(&self) -> &TokenType {
		&self.token_type
	}

	pub fn symbol(&self) -> &String {
		&self.symbol
	}
}

pub struct Lexer {
	text: Vec<char>,
	pos: i32,
	current_char: Option<char>
}

impl Lexer {
	fn new(text: &str) -> Self {
		let mut lexer = Self {
			text: text.to_owned().chars().collect(),
			pos: -1,
			current_char: None
		};

		lexer.advance();
		lexer
	}

	pub fn parse(text: &str) -> Result<Vec<Token>, Error> {
		let mut lexer = Self::new(text);
		lexer.make_tokens()
	}

	fn advance(&mut self) {
		self.pos += 1;

		if self.pos < self.text.len() as i32 {
			self.current_char = Some(*self.text.get(self.pos as usize).unwrap());
		} else {
			self.current_char = None;
		}
	}

	fn make_tokens(&mut self) -> Result<Vec<Token>, Error> {
		let operators = vec![
			// Arithmetic
			"+", "-", "*", "/", "%"
		];

		let mut tokens = Vec::new();

		while self.current_char != None {
			let c = self.current_char.unwrap();
			let str_c = c.to_string();

			if " \t".contains(&str_c) {
				self.advance();
			} else if c.is_digit(10) {
				tokens.push(self.make_number());	
			} else if operators.contains(&str_c.as_str()) {
				tokens.push(Token::new(TokenType::Operator, str_c.as_str()));
				self.advance();
			} else {
				self.advance();
				return Err(IllegalCharError::new(format!("'{}'", c).as_str()));
			}
		}

		Ok(tokens)
	}

	fn make_number(&mut self) -> Token {
		let mut num_str = String::new();
		let mut dot_count = 0;

		while self.current_char != None && (self.current_char.unwrap().is_digit(10) || self.current_char.unwrap() == '.') {
			let c = self.current_char.unwrap();

			if c == '.' {
				if dot_count == 1 {
					break;
				}

				dot_count += 1;
				num_str.push('.');
			} else {
				num_str.push(c);
			}

			self.advance()
		}

		Token::new(TokenType::Literal, &num_str)
	}
}