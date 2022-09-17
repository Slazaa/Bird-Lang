use std::path::{Path, PathBuf};

use super::feedback::*;
use super::constants::*;

/// `TokenType`s are constituent of `Token`s
/// so they can be differentiated easily.
#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
	Literal,
	Identifier,
	Operator,
	Separator,
	Keyword
}

#[derive(Clone, Debug)]
pub enum PathOrText {
	Path(PathBuf),
	Text(String)
}

/// `Position`s keep track of the different `Token`s
/// so they can be identified easily.
#[derive(Clone, Debug)]
pub struct Position {
	index: usize,
	line: usize,
	colomn: usize,
	path_or_text: PathOrText
}

impl Position {
	/// Constructs a new `Position`.
	pub fn new(index: usize, line: usize, colomn: usize, path_or_text: PathOrText) -> Self {
		Self {
			index,
			line,
			colomn,
			path_or_text
		}
	}

	/// Returns the index the `Position` is tracking.
	pub fn index(&self) -> usize {
		self.index
	}

	/// Returns the line the `Position` is tracking.
	pub fn line(&self) -> usize {
		self.line
	}

	/// Returns the column the `Position` is tracking.
	pub fn colomn(&self) -> usize {
		self.colomn
	}

	/// Returns the filename or the text the `Position` is tracking.
	pub fn file_or_text(&self) -> &PathOrText {
		&self.path_or_text
	}

	/// Increments the `Position` index and the collumn and
	/// increments the line when encontering a new line character,
	/// this will also resets the collumn.
	pub fn advance(&mut self, current_char: char) {
		self.index += 1;
		self.colomn += 1;

		if current_char == '\n' {
			self.line += 1;
			self.colomn = 0;
		}
	}
}

/// `Token`s are units for the `Lexer`.
#[derive(Clone, Debug)]
pub struct Token {
	token_type: TokenType,
	symbol: String,
	pos: (Position, Position)
}

impl Token {
	/// Constructs a new `Token`.
	pub fn new(token_type: TokenType, symbol: &str, pos_start: &Position, pos_end: Option<&Position>) -> Self {
		let pos_end = match pos_end {
			Some(x) => x,
			None => pos_start
		};

		Self {
			token_type,
			symbol: symbol.to_owned(),
			pos: (pos_start.clone(), pos_end.clone())
		}
	}

	/// Returns the token type of the `Token`.
	pub fn token_type(&self) -> &TokenType {
		&self.token_type
	}

	/// Returns the symbol of the `Token`.
	pub fn symbol(&self) -> &str {
		&self.symbol
	}

	/// Returns the position of the `Token`.
	pub fn pos(&self) -> (&Position, &Position) {
		(&self.pos.0, &self.pos.1)
	}
}

/// The `Lexer` reads the code and split it into `Token`s.
pub struct Lexer {
	text: String,
	pos: Position,
	current_char: char
}

impl Lexer {
	/// Parse the code and return a `Vec` of `Token` if the code respects the lexing rules,
	/// else return a `Feedback`.
	pub fn parse(text: &str, file_path: Option<&Path>) -> Result<Vec<Token>, Feedback> {
		let file_or_text = match file_path {
			Some(file_path) => PathOrText::Path(file_path.to_path_buf()),
			None => PathOrText::Text(text.to_owned())
		};

		let first_char = match text.chars().next() {
			Some(x) => x,
			None => return Ok(vec![])
		};

		let mut lexer = Self {
			text: text.to_owned(),
			pos: Position::new(0, 0, 0, file_or_text),
			current_char: first_char
		};

		let mut tokens = Vec::new();

		loop {
			if !lexer.is_more_char() {
				break;
			}

			match lexer.current_char {
				' ' | '\r' | '\t' => {
					if lexer.advance().is_err() {
						break;
					}
				}
				'\n' => {
					tokens.push(Token::new(TokenType::Separator, "\n", &lexer.pos, Some(&lexer.pos)));

					if lexer.advance().is_err() {
						break;
					}
				}
				'#' => lexer.skip_comment(),
				'\'' => tokens.push(lexer.make_char()?),
				'"' => tokens.push(lexer.make_string()?),
				_ => {
					if lexer.current_char.is_ascii_digit() {
						tokens.push(lexer.make_number());	
					} else if lexer.current_char.is_alphabetic() || lexer.current_char == '_' {
						tokens.push(lexer.make_identifier());
					} else {
						tokens.push(lexer.make_operator()?);
					}
				}
			}
		}

		Ok(tokens)
	}

	pub fn is_more_char(&self) -> bool {
		self.pos.index() < self.text.len()
	}

	/// Sets the current char to be the next char and returns it if it exists,
	/// else returns `None` and sets the current char to `None`.
	fn advance(&mut self) -> Result<(), ()> {
		self.pos.advance(self.current_char);

		if self.pos.index() < self.text.len() {
			self.current_char = self.text.chars().nth(self.pos.index() as usize).unwrap();
			return Ok(());
		}

		Err(())
	}

	/// Skips the comments, comments starting with a '#' character
	fn skip_comment(&mut self) {
		while self.current_char != '\n' {
			if self.advance().is_err() {
				break;
			}
		}
	}

	/// Constructs a number `Token`
	fn make_number(&mut self) -> Token {
		let mut res = String::new();
		let mut dot_count = 0;

		let pos_start = self.pos.clone();
		let mut pos_end = pos_start.clone();

		loop {
			if !self.current_char.is_ascii_digit() && self.current_char != '.' && self.current_char != '_' {
				break;
			}

			if self.current_char == '.' {
				if dot_count == 1 {
					break;
				}

				dot_count += 1;
				res.push('.');
			} else {
				res.push(self.current_char);
			}

			pos_end = self.pos.clone();
			
			if self.advance().is_err() {
				break;
			}
		}

		Token::new(TokenType::Literal, &res, &pos_start, Some(&pos_end))
	}

	fn escape_sequance(&mut self) -> Result<char, Feedback> {
		let pos_start = self.pos.clone();

		Ok(match self.current_char {
			'b' => 0x08 as char,
			'n' => 0x0A as char,
			'r' => 0x0D as char,
			't' => 0x09 as char,
			'v' => 0x0B as char,
			'\\' | '\'' | '"' => self.current_char,
			_ => return Err(Error::invalid_syntax(Some((&pos_start, &self.pos)), &format!("Invalid escape sequance \\{}", self.current_char)))
		})
	}

	/// Constructs a identifier `Token`
	fn make_identifier(&mut self) -> Token {
		let mut res = String::new();

		let pos_start = self.pos.clone();
		let mut pos_end = pos_start.clone();

		loop {
			if !self.current_char.is_alphanumeric() && self.current_char != '_' {
				break;
			}

			res.push(self.current_char);
			pos_end = self.pos.clone();
			
			if self.advance().is_err() {
				break;
			}
		}

		let token_type = match KEYWORDS.contains(&res.as_str()) {
			true => TokenType::Keyword,
			false => {
				match ["false", "true", "null"].contains(&res.as_str()) {
					true => TokenType::Literal,
					false => TokenType::Identifier
				}
			}
		};

		Token::new(token_type, &res, &pos_start, Some(&pos_end))
	}

	fn make_char(&mut self) -> Result<Token, Feedback> {
		let mut res = String::new();
		res.push('\'');

		let pos_start = self.pos.clone();

		if self.advance().is_err() {
			return Err(Error::expected((&pos_start, &self.pos.clone()), "char", None));
		}

		match self.current_char {
			'\'' => return Err(Error::expected((&pos_start, &self.pos.clone()), "char", Some("'"))),
			_ => {
				if self.current_char == '\\' {
					if self.advance().is_err() {
						return Err(Error::expected((&pos_start, &self.pos.clone()), "char", None));
					}

					res.push(self.escape_sequance()?);
				} else {
					res.push(self.current_char);
				}
			}
		}

		if self.advance().is_err() {
			return Err(Error::expected((&pos_start, &self.pos.clone()), "'", None));
		}

		match self.current_char {
			'\'' => res.push('\''),
			_ => return Err(Error::expected((&pos_start, &self.pos.clone()), "char", Some(&self.current_char.to_string())))
		}

		self.advance().unwrap_or(());
		Ok(Token::new(TokenType::Literal, &res, &pos_start, Some(&self.pos.clone())))
	}

	/// Constructs a string `Token`
	fn make_string(&mut self) -> Result<Token, Feedback> {
		let mut res = String::new();
		res.push('"');

		let pos_start = self.pos.clone();
		let mut pos_end = pos_start.clone();

		loop {
			if self.advance().is_err() {
				break;
			}

			if self.current_char == '\\' {
				if self.advance().is_err() {
					return Err(Error::expected((&pos_start, &self.pos.clone()), "char", None));
				}

				res.push(self.escape_sequance()?);
			} else {
				res.push(self.current_char);
			}

			pos_end = self.pos.clone();

			if self.current_char == '"' {
				break;
			}
		}

		self.advance().unwrap_or(());

		Ok(Token::new(TokenType::Literal, &res, &pos_start, Some(&pos_end)))
	}

	/// Constructs an operator `Token`, it calls the function `make_separator` if it fails 
	fn make_operator(&mut self) -> Result<Token, Feedback> {
		let mut res = String::new();

		let pos_start = self.pos.clone();
		let mut pos_end = pos_start.clone();

		if !OPERATOR_CHARS.contains(self.current_char) {
			return self.make_separator();
		}

		loop {
			if !OPERATOR_CHARS.contains(self.current_char) {
				break;
			}

			res.push(self.current_char);
			pos_end = self.pos.clone();

			if self.advance().is_err() {
				break;
			}
		}

		if !OPERATORS.contains(&res.as_str()) {
			return self.make_separator();
		}

		Ok(Token::new(TokenType::Operator, &res, &pos_start, Some(&pos_end)))
	}

	/// Constructs a separator `Token`
	fn make_separator(&mut self) -> Result<Token, Feedback> {
		let pos_start = self.pos.clone();

		if !SEPARATORS.contains(self.current_char) {
			return Err(Error::invalid_syntax(Some((&pos_start, &pos_start)), "Invalid token"));
		}

		let res = Token::new(TokenType::Separator, &String::from(self.current_char), &pos_start, Some(&pos_start));

		self.advance().unwrap_or(());
		Ok(res)
	}
}