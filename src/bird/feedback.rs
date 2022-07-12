use std::fmt::{self, Display};
use std::fs::File;
use std::io::{BufReader, BufRead};

use super::lexer::Position;

pub enum FeedbackType {
	Info,
	Warning,
	Error
}

impl Display for FeedbackType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let symbol = match self {
			Self::Info => "Info",
			Self::Warning => "Warning",
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
	pub fn new(feedback_type: FeedbackType, position: Option<(Position, Position)>, description: &str) -> Self {
		Self {
			feedback_type,
			position,
			description: description.to_owned()
		}
	}

	pub fn as_string(&self) -> String {
		let mut result = String::new();

		result.push_str(format!("{}: {}", self.feedback_type, self.description).as_str());

		if let Some((pos_start, _pos_end)) = &self.position {
			let line_string = format!("{}", pos_start.line() + 1);

			result.push_str(format!("\n  --> {}:{}:{}", pos_start.filname(), line_string, pos_start.colomn() + 1).as_str());

			let mut pipe: String = (0..=line_string.len()).map(|_| ' ')
				.collect();
			
			pipe.push('|');

			let mut pipe_line = String::from(" |");

			pipe_line.insert_str(0, &line_string);

			let file = File::open(pos_start.filname()).unwrap();
			let reader = BufReader::new(file);

			let line_text = reader.lines()
				.nth(pos_start.line() as usize)
				.unwrap()
				.unwrap();

			result.push_str(&format!("\n{}", pipe));
			result.push_str(&format!("\n{} {}", pipe_line, line_text));
			result.push_str(&format!("\n{}", pipe));
		}

		result
	}
}

pub struct Info;

impl Info {

}

pub struct Warning;

impl Warning {

}

pub struct Error;

impl Error {
	pub fn illegal_char(position: (Position, Position), description: &str) -> Feedback {
		Feedback::new(FeedbackType::Error, Some(position), description)
	}

	pub fn invalid_syntax(position: Option<(Position, Position)>, description: &str) -> Feedback {
		Feedback::new(FeedbackType::Error, position, description)
	}

	pub fn no_input_file() -> Feedback {
		Feedback::new(FeedbackType::Error, None, "No input file")
	}

	pub fn no_file_or_dir(filename: &str) -> Feedback {
		Feedback::new(FeedbackType::Error, None, &format!("No such file or directory '{}'", filename))
	}
}