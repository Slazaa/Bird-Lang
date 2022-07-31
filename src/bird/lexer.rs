use std::fs;

use super::feedback::*;
use super::constants::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum TokenType {
	Literal,
	Operator,
	Separator,
	Keyword,
	Identifier
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
	pub fn new(token_type: TokenType, symbol: &str, pos_start: &Position, pos_end: Option<&Position>) -> Self {
		Self {
			token_type,
			symbol: symbol.to_owned(),
			pos_start: pos_start.clone(),
			pos_end: match pos_end {
				Some(x) => x.clone(),
				None => pos_start.clone()
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

	pub fn pos(&self) -> (&Position, &Position) {
		(&self.pos_start, &self.pos_end)
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

		let mut tokens = Vec::new();

		while let Some(c) = lexer.current_char {
			let str_c = c.to_string();

			if " \r\t".contains(&str_c) {
				lexer.advance();
			} else if "\n".contains(&str_c) {
				let pos = lexer.pos.clone();
				tokens.push(Token::new(TokenType::Separator, "\n", &pos, Some(&pos)));
				lexer.advance();
			} else if "#".contains(&str_c) {
				lexer.skip_comment();
			} else if c.is_ascii_digit() {
				tokens.push(lexer.make_number());	
			} else if c.is_alphabetic() || c == '_' {
				tokens.push(lexer.make_identifier());
			} else if "\"".contains(&str_c) {
				match lexer.make_string() {
					Ok(token) => tokens.push(token),
					Err(e) => return Err(e)
				}
			} else {
				match lexer.make_operator() {
					Ok(token) => tokens.push(token),
					Err(e) => return Err(e)
				}
			}
		}

		Ok(tokens)
	}

	fn advance(&mut self) -> Option<char> {
		self.pos.advance(self.current_char);

		if self.pos.index() < self.text.len() as i32 {
			self.current_char = Some(self.text.chars().nth(self.pos.index() as usize).unwrap());
			
			if let Some(current_char) = self.current_char {
				return Some(current_char);
			}
		}

		self.current_char = None;

		None
	}

	fn skip_comment(&mut self) {
		while let Some(c) = self.current_char {
			if c == '\n' {
				break;
			}

			self.advance();
		}
	}

	fn make_number(&mut self) -> Token {
		let mut res = String::new();
		let mut dot_count = 0;

		let pos_start = self.pos.clone();
		let mut pos_end = pos_start.clone();

		while let Some(current_char) = self.current_char {
			if !current_char.is_ascii_digit() && current_char != '.' {
				break;
			}

			let c = current_char;

			if c == '.' {
				if dot_count == 1 {
					break;
				}

				dot_count += 1;
				res.push('.');
			} else {
				res.push(c);
			}

			pos_end = self.pos.clone();
			self.advance();
		}

		Token::new(TokenType::Literal, &res, &pos_start, Some(&pos_end))
	}

	fn make_identifier(&mut self) -> Token {
		let mut res = String::new();

		let pos_start = self.pos.clone();
		let mut pos_end = pos_start.clone();

		while let Some(current_char) = self.current_char {
			if !current_char.is_alphanumeric() && current_char != '_' {
				break;
			}

			res.push(current_char);

			pos_end = self.pos.clone();
			self.advance();
		}

		let token_type = if KEYWORDS.contains(&res.as_str()) {
			TokenType::Keyword
		} else {
			TokenType::Identifier
		};

		Token::new(token_type, &res, &pos_start, Some(&pos_end))
	}

	fn make_string(&mut self) -> Result<Token, Feedback> {
		let mut res = String::new();
		res.push('"');

		let pos_start = self.pos.clone();
		let mut pos_end = pos_start.clone();

		loop {
			let c = match self.advance() {
				Some(x) => x,
				None => return Err(Error::expected((&pos_end, &pos_end), "'\"'", None))
			};

			res.push(c);
			pos_end = self.pos.clone();

			if c == '"' {
				break;
			}
		}

		self.advance();

		Ok(Token::new(TokenType::Literal, &res, &pos_start, Some(&pos_end)))
	}

	fn make_operator(&mut self) -> Result<Token, Feedback> {
		let mut res = String::new();

		let pos_start = self.pos.clone();
		let mut pos_end = pos_start.clone();

		if !OPERATOR_CHARS.contains(self.current_char.unwrap()) {
			return self.make_separator();
		}

		while let Some(current_char) = self.current_char {
			if !OPERATOR_CHARS.contains(current_char) {
				break;
			}

			res.push(current_char);
			pos_end = self.pos.clone();

			self.advance();
		}

		if !OPERATORS.contains(&res.as_str()) {
			return self.make_separator();
		}

		Ok(Token::new(TokenType::Operator, &res, &pos_start, Some(&pos_end)))
	}

	fn make_separator(&mut self) -> Result<Token, Feedback> {
		let pos_start = self.pos.clone();

		if !SEPARATORS.contains(self.current_char.unwrap()) {
			return Err(Error::invalid_syntax(Some((&pos_start, &pos_start)), "Invalid token"));
		}

		let current_char = self.current_char.unwrap();

		self.advance();

		Ok(Token::new(TokenType::Separator, &String::from(current_char), &pos_start, Some(&pos_start)))
	}
}