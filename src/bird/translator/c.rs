use std::fs;

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

		translator.advance();

		let mut index_offset = 0;

		while let Some(token_buffer) = &translator.token_buffer {
			let mut text_range = 0..=0;

			if !token_buffer.is_empty() {
				text_range = (token_buffer.first().unwrap().pos_start().index() + index_offset) as usize..=(token_buffer.last().unwrap().pos_end().index() + index_offset) as usize;
			}

			let token_buffer_clone = token_buffer.clone();
			let pattern_found = PatternFinder::find(&token_buffer_clone, PatternContext::Global);

			match pattern_found {
				Pattern::VarDecl { identifier, var_type } => {
					let result = format!("{} {}", var_type, identifier);
					index_offset += result.len() as i32 - (text_range.end() - text_range.start()) as i32 - 1;

					translator.result.replace_range(text_range, &result);
					translator.clear_token_buffer();
				}
				Pattern::MembDecl { .. } => (),
				Pattern::FuncDecl { identifier, return_type, params, public } => {
					let return_type_str;

					if identifier == "main" {
						return_type_str = "int32".to_owned();
					} else {
						return_type_str = match return_type {
							Some(x) => x,
							None => "void".to_owned()
						};
					}

					let params = match params.is_empty() {
						true => "void".to_owned(),
						false => "".to_owned()
					};

					let public_str = match public {
						true => "".to_owned(),
						false => "static ".to_owned()
					};

					let result = format!("{}{} {}({})", public_str, return_type_str, identifier, params);
					index_offset += result.len() as i32 - (text_range.end() - text_range.start()) as i32 - 1;

					translator.result.replace_range(text_range, &result);
					translator.clear_token_buffer();
				}
				Pattern::Ignored => translator.clear_token_buffer(),
				Pattern::Invalid => ()
			}

			translator.advance();
		}
		
		Ok(translator.result)
	}

	fn clear_token_buffer(&mut self) {
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