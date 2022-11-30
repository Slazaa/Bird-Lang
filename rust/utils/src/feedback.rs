use std::fmt::{self, Display, Write};
use std::fs::File;
use std::io::{BufRead, BufReader};

use parse::Loc;

#[derive(Clone)]
pub enum FeedbackType {
	Error
}

impl Display for FeedbackType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", match self {
			Self::Error => "Error"
		})
	}
}

#[derive(Clone)]
pub struct Feedback {
	feedback_type: FeedbackType,
	loc: Option<Loc>,
	desc: String,
}

impl Feedback {
	pub fn new(feedback_type: FeedbackType, loc: Option<&Loc>, desc: &str) -> Self {
		Self {
			feedback_type,
			loc: loc.map(|x| x.to_owned()),
			desc: desc.to_owned()
		}
	}

	pub fn desc(&self) -> &String {
		&self.desc
	}

	fn arrow_pos(location: &Loc) -> String {
		let mut result = String::new();
		let line_string = format!("{}", location.start.line + 1);

		if let Some(filename) = &location.filename {
			write!(result, "\n  --> {}:{}:{}", filename, line_string, location.start.col + 1).unwrap();
		}

		let mut pipe: String = (0..=line_string.len()).map(|_| ' ').collect();
		pipe.push('|');

		let mut pipe_line = String::from(" |");
		pipe_line.insert_str(0, &line_string);

		let mut pipe_down = pipe.clone();

		let line_text = match &location.filename {
			Some(filename) => {
				let file = File::open(filename).unwrap();
				let reader = BufReader::new(file);

				reader.lines()
					.nth(location.start.line)
					.unwrap()
					.unwrap()
			}
			None => todo!()
		};

		for i in 0..location.start.col {
			match line_text.chars().nth(i as usize) {
				Some(c) if c == '\t' => pipe_down.push('\t'),
				_ => pipe_down.push(' ')
			}
		}

		for _ in location.start.col..=location.end.col {
			pipe_down.push('^');
		}

		write!(result, "\n{}", pipe).unwrap();
		write!(result, "\n{} {}", pipe_line, line_text).unwrap();
		write!(result, "\n{}", pipe_down).unwrap();

		result
	}
}

pub struct Error;

impl Error {
	pub fn expected(loc: &Loc, expected: &str, found: Option<&str>) -> Feedback {
		let mut expected = expected;

		if expected.contains('\n') {
			expected = "new line";
		}

		let description = match found {
			Some(found) => {
				let mut found = found;

				if found.contains('\n') {
					found = "new line";
				}

				format!("Expected {}, found {}", expected, found)
			}
			None => format!("Expected {}", expected)
		};

		Feedback::new(FeedbackType::Error, Some(loc), &description)
	}

	pub fn invalid_syntax(loc: Option<&Loc>, description: &str) -> Feedback {
		Feedback::new(FeedbackType::Error, loc, description)
	}

	pub fn no_file_or_dir(filename: &str) -> Feedback {
		Feedback::new(FeedbackType::Error, None, &format!("No such file or directory '{}'", filename))
	}
/*
	pub fn redefinition(position: (&Position, &Position), identifier: &str) -> Feedback {
		Feedback::new(FeedbackType::Error,Some(position),&format!("Redefinition of '{}'", identifier))
	}
*/
	pub fn unexpected(loc: &Loc, unexpected: &str) -> Feedback {
		Feedback::new(FeedbackType::Error,Some(&loc),&format!("Unexpected {}", unexpected))
	}

	pub fn unspecified(description: &str) -> Feedback {
		Feedback::new(FeedbackType::Error, None, description)
	}
}

impl Display for Feedback {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
/*
		let mut res = String::new();
		res.push_str(format!("{}: {}", self.feedback_type, self.desc).as_str());

		if let Some(loc) = &self.loc {
			res.push_str(&Self::arrow_pos(loc));
		}

		write!(f, "{}", res)
*/
		println!("Test!");
	
		if let Some(loc) = &self.loc {
			write!(f, "{}", Self::arrow_pos(loc))
		} else {
			write!(f, "{}: {}", self.feedback_type, self.desc)
		}
	}
}