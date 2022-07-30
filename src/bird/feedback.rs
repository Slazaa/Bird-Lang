use std::fmt::{self, Display, Write};
use std::fs::File;
use std::io::{BufReader, BufRead};

use super::lexer::Position;

pub enum FeedbackType {
	Error
}

impl Display for FeedbackType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let symbol = match self {
			Self::Error => "Error"
		};

		write!(f, "{}", symbol)
	}
}

pub struct Feedback {
	feedback_type: FeedbackType,
	position: Option<(Position, Position)>,
	description: String
}

impl Feedback {
	pub fn new(feedback_type: FeedbackType, position: Option<(&Position, &Position)>, description: &str) -> Self {
		Self {
			feedback_type,
			position: position.map(|(pos_start, pos_end)| (pos_start.clone(), pos_end.clone())),
			description: description.to_owned()
		}
	}

	fn arrow_pos(pos_start: &Position, pos_end: &Position) -> String {
		let mut result = String::new();

		let line_string = format!("{}", pos_start.line() + 1);
		write!(result, "\n  --> {}:{}:{}", pos_start.filname(), line_string, pos_start.colomn() + 1).unwrap();

		let mut pipe: String = (0..=line_string.len()).map(|_| ' ')
			.collect();

		pipe.push('|');

		let mut pipe_line = String::from(" |");
		pipe_line.insert_str(0, &line_string);

		let mut pipe_down = pipe.clone();
		let file = File::open(pos_start.filname()).unwrap();
		let reader = BufReader::new(file);

		let line_text = reader.lines()
			.nth(pos_start.line() as usize)
			.unwrap()
			.unwrap();

		for i in 0..pos_start.colomn() {
			match line_text.chars().nth(i as usize).unwrap() {
				'\t' => pipe_down.push_str("\t"),
				_ => pipe_down.push(' ')
			}
		}

		for _ in pos_start.colomn()..=pos_end.colomn() {
			pipe_down.push('^');
		}

		write!(result, "\n{}", pipe).unwrap();
		write!(result, "\n{} {}", pipe_line, line_text).unwrap();
		write!(result, "\n{}", pipe_down).unwrap();

		result
	}

	pub fn as_string(&self) -> String {
		let mut result = String::new();

		result.push_str(format!("{}: {}", self.feedback_type, self.description).as_str());

		if let Some((pos_start, pos_end)) = &self.position {
			result.push_str(&Self::arrow_pos(pos_start, pos_end));
		}

		result
	}
}

pub struct Error;

impl Error {
	pub fn expected(position: (&Position, &Position), expected: &str, found: Option<&str>) -> Feedback {
		let mut expected = expected;

		if expected.contains("\n") {
			expected = "new line";
		}

		let description = match found {
			Some(found) => {
				let mut found = found;

				if found.contains("\n") {
					found = "new line";
				}

				format!("Expected {}, found {}", expected, found)
			}
			None => format!("Expected {}", expected)
		};

		Feedback::new(FeedbackType::Error, Some(position), &description)
	}
	
	pub fn illegal_char(position: (&Position, &Position), character: char) -> Feedback {
		Feedback::new(FeedbackType::Error, Some(position), &format!("Illegal character '{}'", character))
	}

	pub fn invalid_syntax(position: Option<(&Position, &Position)>, description: &str) -> Feedback {
		Feedback::new(FeedbackType::Error, position, description)
	}

	pub fn no_input_file() -> Feedback {
		Feedback::new(FeedbackType::Error, None, "No input file")
	}

	pub fn no_file_or_dir(filename: &str) -> Feedback {
		Feedback::new(FeedbackType::Error, None, &format!("No such file or directory '{}'", filename))
	}

	pub fn unexpected(position: (&Position, &Position), unexpected: &str) -> Feedback {
		Feedback::new(FeedbackType::Error, Some(position), &format!("Unexpected {}", unexpected))
	}

	pub fn unspecified(description: &str) -> Feedback {
		Feedback::new(FeedbackType::Error, None, description)
	}
}