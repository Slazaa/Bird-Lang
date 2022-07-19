use super::lexer::{Token, TokenType};

pub enum Pattern {
	VarDecl {
		identifier: String,
		var_type: String
	},
	MembDecl {
		identifier: String,
		var_type: String
	},
	FuncDecl {
		identifier: String,
		return_type: Option<String>,
		params: Vec<(String, String)>,
		public: bool
	},
	Ignored,
	Invalid
}

struct PatternUnit {
	token_type: TokenType,
	symbol: Option<String>
}

impl PatternUnit {
	pub fn new(token_type: TokenType, symbol: Option<&str>) -> Self {
		Self {
			token_type,
			symbol: match symbol {
				Some(x) => Some(x.to_owned()),
				None => None
			}
		}
	}

	pub fn from_tuple(info: (TokenType, Option<&str>)) -> Self {
		Self::new(info.0, info.1)
	}

	pub fn match_token(&self, token: &Token) -> bool {
		if *token.token_type() == self.token_type && (!self.symbol.is_some() || token.symbol() == self.symbol.as_ref().unwrap()) {
			return true;
		}

		false
	}
}

macro_rules! pattern_template {
	($($x:expr),*) => {
		{
			let mut temp_pattern = Vec::new();
			$(
				temp_pattern.push(PatternUnit::from_tuple($x));
			)*
			temp_pattern
		}
	};
}

pub enum PatternContext {
	Global
}

pub struct PatternFinder;

impl PatternFinder {
	pub fn find(tokens: &Vec<Token>, _context: PatternContext) -> Pattern {
		if tokens.is_empty() {
			return Pattern::Invalid;
		}

		match tokens.first().unwrap().token_type() {
			TokenType::Operator | TokenType::Separator => return Pattern::Ignored,
			_ => ()
		}

		let patterns = vec![
			("VarDecl", pattern_template!((TokenType::Keyword, Some("var")), (TokenType::Identifier, None), (TokenType::Operator, Some(":")), (TokenType::Identifier, None))),
			("MembDecl", pattern_template!((TokenType::Identifier, None), (TokenType::Operator, Some(":")), (TokenType::Identifier, None))),
			("FuncDecl", pattern_template!((TokenType::Keyword, Some("pub")), (TokenType::Keyword, Some("func")), (TokenType::Identifier, None), (TokenType::Separator, Some("(")), (TokenType::Separator, Some(")")))),
			("FuncDecl", pattern_template!((TokenType::Keyword, Some("func")), (TokenType::Identifier, None), (TokenType::Separator, Some("(")), (TokenType::Separator, Some(")"))))
		];

		for (pattern_type, pattern) in patterns {
			match Self::match_pattern(&tokens, &pattern) {
				Ok(_) => return Self::make_pattern(&tokens, pattern_type),
				Err(_) => ()
			}
		}

		return Pattern::Invalid;
	}

	fn match_pattern(tokens: &Vec<Token>, pattern: &Vec<PatternUnit>) -> Result<(), Option<usize>> {
		if tokens.len() != pattern.len() {
			return Err(None);
		}

		for (i, token) in tokens.iter().enumerate() {
			if !pattern[i].match_token(token) {
				return Err(Some(i));
			}
		}

		Ok(())
	}

	fn make_pattern(tokens: &Vec<Token>, pattern_type: &str) -> Pattern {
		match pattern_type {
			"VarDecl" => Pattern::VarDecl {
				identifier: tokens[1].symbol().to_owned(),
				var_type: tokens[3].symbol().to_owned()
			},
			"FuncDecl" if tokens[0].symbol() == "pub" => Pattern::FuncDecl {
				identifier: tokens[2].symbol().to_owned(),
				return_type: None,
				params: Vec::new(),
				public: true
			},
			"FuncDecl" => Pattern::FuncDecl {
				identifier: tokens[1].symbol().to_owned(),
				return_type: None,
				params: Vec::new(),
				public: false
			},
			_ => Pattern::Invalid
		}
	}
}