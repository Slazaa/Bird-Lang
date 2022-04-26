use std::fs;

#[derive(PartialEq, Clone, Copy)]
pub enum TokenType {
	None,
	Identifier,
	Keyword,
	Separator,
	Operator,
	Literal
}

pub struct Token {
	token_type: TokenType,
	line: u32,
	colomn: u32,
	symbol: String
}

impl Token {
	pub fn new(token_type: TokenType, line: u32, colomn: u32) -> Self {
		Token {
			token_type,
			line,
			colomn,
			symbol: String::new()
		}
	}

	pub fn token_type(&self) -> &TokenType {
		&self.token_type
	}

	pub fn line(&self) -> &u32 {
		&self.line
	}

	pub fn colomn(&self) -> &u32 {
		&self.colomn
	}

	pub fn symbol(&self) -> &String {
		&self.symbol
	}

	pub fn mut_token_type(&mut self) -> &mut TokenType {
		&mut self.token_type
	}

	pub fn mut_symbol(&mut self) -> &mut String {
		&mut self.symbol
	}
}

#[derive(PartialEq)]
enum CommentType {
	None,
	SingleLine,
	MultiLine
}

pub fn tokenize_file(filename: &str) -> Result<Vec<Token>, String> {
	let keywords = vec![
		"break"   , "case"   , "const" ,
		"continue", "default", "else"  ,
		"enum"    , "for"    , "func"  ,
		"if"      , "import" , "return",
		"struct"  , "switch" , "var"   ,
		"while"
	];

	let operators = vec![
		// Arithmetic
		"+", "-", "*", "/", "%",

		// Relational
		"==", "!=", ">", "<", ">=", "<=",

		// Logical
		"&&", "||", "!",

		// Bitwise
		"&", "|", "^", "~", "<<", ">>",

		// Assignment
		"=", "+=", "-=", "*=", "/=", "%=", "<<=", ">>=", "&=", "^=", "|=",

		// Function
		"->"
	];

	let separators = "(),.:;[]{}";

	let file_content = fs::read_to_string(filename).expect("Failed to read the file");
	let mut tokens: Vec<Token> = Vec::new();

	let mut last_char = '\0';

	let mut colomn = 1;
	let mut line = 1;

	let mut comment_type = CommentType::None;
	let mut string_state = false;
	let mut float_dot_found = false;

	for c in file_content.chars() {
		if c == '\n' {
			colomn = 0;
			line += 1;
		}

		if (comment_type == CommentType::SingleLine && c == '\n') || (comment_type == CommentType::MultiLine && last_char == '*' && c == '/') {
			comment_type = CommentType::None;
			continue;
		}

		if comment_type == CommentType::None {
			if last_char == '/' {
				if c == '/' {
					comment_type = CommentType::SingleLine;
				} else if c == '*' {
					comment_type = CommentType::MultiLine;
				}

				continue;
			}

			if !string_state {
				if last_char != '\n' && last_char != '\t' && last_char != ' ' {
					let mut split_token = false;
					let last_token = tokens.last_mut();

					if last_token.is_none() {
						split_token = true;
					} else if let Some(last_token) = last_token {
						if
							(
								*last_token.token_type() == TokenType::None &&
								(c.is_alphanumeric() || c == '_' || separators.contains(c))
							) ||
							(
								*last_token.token_type() == TokenType::Identifier &&
								(!c.is_alphanumeric() && c != '_')
							) ||
							(
								*last_token.token_type() == TokenType::Literal &&
								!c.is_digit(10) &&
								(c != '.' || (c == '.' && float_dot_found))
							) ||
							*last_token.token_type() == TokenType::Separator
						{
							split_token = true;

							if *last_token.token_type() == TokenType::Identifier {
								if keywords.contains(&last_token.symbol().as_str()) {
									*last_token.mut_token_type() = TokenType::Keyword;
								} else if last_token.symbol() == "false" || last_token.symbol() == "true" {
									*last_token.mut_token_type() = TokenType::Literal;
								}
							} else if *last_token.token_type() == TokenType::None && operators.contains(&last_token.symbol().as_str()) {
								*last_token.mut_token_type() = TokenType::Operator;
							}
						}
					}

					if split_token {
						if float_dot_found {
							float_dot_found = false;
						}

						let mut token_type = TokenType::None;

						if c.is_alphabetic() || c == '_' {
							token_type = TokenType::Identifier;
						} else if c.is_digit(10) {
							token_type = TokenType::Literal;
						} else if separators.contains(c) {
							token_type = TokenType::Separator;
						}

						tokens.push(Token::new(token_type, line, colomn));
					}

					let last_token = tokens.last_mut().unwrap();

					if *last_token.token_type() == TokenType::Literal && c == '.' && !float_dot_found {
						float_dot_found = true;
					}

					if c == '"' {
						string_state = true;
					}

					last_token.mut_symbol().push(c);
				}
			} else {
				let last_token = tokens.last_mut().unwrap();

				if c == '"' {
					*last_token.mut_token_type() = TokenType::Literal;
					string_state = false;
				}

				last_token.mut_symbol().push(c);
			}
		}

		last_char = c;
		colomn += 1;
	}

	let last_token = tokens.last_mut().unwrap();

	if *last_token.token_type() == TokenType::Identifier {
		if keywords.contains(&last_token.symbol().as_str()) {
			*last_token.mut_token_type() = TokenType::Keyword;
		} else if last_token.symbol() == "false" || last_token.symbol() == "true" {
			*last_token.mut_token_type() = TokenType::Literal;
		}
	} else if *last_token.token_type() == TokenType::None && operators.contains(&last_token.symbol().as_str()) {
		*last_token.mut_token_type() = TokenType::Operator;
	}

	Ok(tokens)
}

pub fn print_tokens(tokens: &Vec<Token>) {
	let types = vec![
		"None",
		"Identifier",
		"Keyword",
		"Separator",
		"Operator",
		"Literal"
	];

	for token in tokens {
		println!("Type: {} Line: {} Colomn: {} Symbol: {}", types[*token.token_type() as usize], token.line(), token.colomn(), token.symbol());
	}
}