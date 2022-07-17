use std::io::Write;
use std::fs;
use std::io;

use crate::bird::feedback::*;
use crate::bird::lexer::Token;
use crate::bird::pattern_finder::{PatternFinder, Pattern, PatternContext};

pub struct Translator {
	tokens: Vec<Token>,
	token_index: i32,
	token_buffer: Option<Vec<Token>>,
	result: String
}

impl Translator {
	pub fn translate(filename: &str, tokens: Vec<Token>) -> Result<String, Feedback> {
		let text = match fs::read_to_string(filename) {
			Ok(x) => x,
			Err(_) => return Err(Error::no_file_or_dir(filename))
		};

		let mut translator = Self {
			tokens,
			token_index: -1,
			token_buffer: Some(Vec::new()),
			result: text
		};

		let mut res = String::new();

		translator.advance();

		while let Some(token_buffer) = &mut translator.token_buffer {
			let mut text_range = None;

			if !token_buffer.is_empty() {
				text_range = Some(token_buffer.first().unwrap().pos_start().index() as usize..=token_buffer.last().unwrap().pos_end().index() as usize);
			}

			let token_buffer_clone = translator.token_buffer.as_ref().unwrap().clone();

			match PatternFinder::find(&token_buffer_clone, PatternContext::Global) {
				Pattern::VarDecl { identifier, var_type } => translator.result.replace_range(text_range.unwrap(), &format!("{} {}", var_type, identifier)),
				Pattern::MembDecl { identifier, var_type } => (),
				Pattern::FuncDecl { identifier, return_type, params, public } => (),
				Pattern::Ignored => translator.add_to_res(&mut res),
				Pattern::Invalid => ()
			}

			translator.advance();
		}
		
		Ok(translator.result)
	}

	fn add_to_res(&mut self, res: &mut String) {
		for token in self.token_buffer.as_ref().unwrap() {
			res.push_str(token.symbol());
			res.push(' ');
		}

		self.token_buffer.as_mut().unwrap().clear();
	}

	fn advance(&mut self) {
		self.token_index += 1;

		if self.token_index < self.tokens.len() as i32 {
			if let Some(token_buffer) = &mut self.token_buffer {
				token_buffer.push(self.tokens[self.token_index as usize].clone());
			}

			return;
		}

		self.token_buffer = None;
	}
}