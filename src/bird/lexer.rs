use std::fs;

use super::feedback::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TokenType {
	Operator,
	Literal
}

#[derive(Clone, Default, Debug)]
pub struct Position {
	index: i32,
	line: i32,
	colomn: i32,
	filename: String
}

impl Position {
	pub fn new(index: i32, line: i32, colomn: i32, filename: &str) -> Self {
		Self {
			index,
			line,
			colomn,
			filename: filename.to_owned()
		}
	}

	pub fn index(&self) -> i32 {
		self.index
	}

	pub fn line(&self) -> i32 {
		self.line
	}

	pub fn colomn(&self) -> i32 {
		self.colomn
	}

	pub fn colomn_mut(&mut self) -> &mut i32 {
		&mut self.colomn
	}

	pub fn filname(&self) -> &str {
		self.filename.as_str()
	}

	pub fn advance(&mut self, current_char: Option<char>) {
		self.index += 1;
		self.colomn += 1;

		if let Some(current_char) = current_char {
			if current_char == '\n' {
				self.line += 1;
				self.colomn = 0;
			}
		}
	}
}

#[derive(Clone, Debug)]
pub struct Token {
	token_type: TokenType,
	symbol: String,
	pos_start: Position,
	pos_end: Position
}

impl Token {
	pub fn new(token_type: TokenType, symbol: &str, pos_start: Position, pos_end: Option<Position>) -> Self {
		Self {
			token_type,
			symbol: symbol.to_owned(),
			pos_start: pos_start.clone(),
			pos_end: match pos_end {
				Some(x) => x,
				None => pos_start
			}
		}
	}

	pub fn token_type(&self) -> &TokenType {
		&self.token_type
	}

	pub fn symbol(&self) -> &str {
		&self.symbol
	}

	pub fn pos_start(&self) -> &Position {
		&self.pos_start
	}

	pub fn pos_end(&self) -> &Position {
		&self.pos_end
	}
}

pub struct Lexer {
	text: String,
	pos: Position,
	current_char: Option<char>
}

impl Lexer {
	fn new(filename: &str) -> Result<Self, Feedback> {
		let text = match fs::read_to_string(filename) {
			Ok(x) => x,
			Err(_) => return Err(Error::no_file_or_dir(filename))
		};

		let mut lexer = Self {
			text,
			pos: Position::new(-1, 0, -1, filename),
			current_char: None
		};

		lexer.advance();
		
		Ok(lexer)
	}

	pub fn parse(filename: &str) -> Result<Vec<Token>, Feedback> {
		let mut lexer = match Self::new(filename) {
			Ok(x) => x,
			Err(e) => return Err(e)
		};

		let operators = vec![
			// Arithmetic
			"+", "-", "*", "/", "%"
		];

		let mut tokens = Vec::new();

		while lexer.current_char != None {
			let c = lexer.current_char.unwrap();
			let str_c = c.to_string();

			if " \n\r\t".contains(&str_c) {
				lexer.advance();
			} else if c.is_digit(10) {
				tokens.push(lexer.make_number());	
			} else if operators.contains(&str_c.as_str()) {
				tokens.push(Token::new(TokenType::Operator, str_c.as_str(), lexer.pos.clone(), None));
				lexer.advance();
			} else {
				let pos_start = lexer.pos.clone();
				lexer.advance();
				return Err(Error::illegal_char((&pos_start, &pos_start), c));
			}
		}

		Ok(tokens)
	}

	fn advance(&mut self) {
		self.pos.advance(self.current_char);

		if self.pos.index() < self.text.len() as i32 {
			self.current_char = Some(self.text.chars().nth(self.pos.index() as usize).unwrap());
		} else {
			self.current_char = None;
		}
	}

	fn make_number(&mut self) -> Token {
		let mut num_str = String::new();
		let mut dot_count = 0;
		let pos_start = self.pos.clone();

		while let Some(current_char) = self.current_char {
			if !current_char.is_digit(10) && current_char != '.' {
				break;
			}

			let c = current_char;

			if c == '.' {
				if dot_count == 1 {
					break;
				}

				dot_count += 1;
				num_str.push('.');
			} else {
				num_str.push(c);
			}

			self.advance();
		}

		Token::new(TokenType::Literal, &num_str, pos_start, Some(self.pos.clone()))
	}
}