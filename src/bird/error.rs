use super::lexer::Position;

pub struct Error {
	position: Option<(Position, Position)>,
	error_name: String,
	details: String
}

impl Error {
	pub fn new(position: Option<(Position, Position)>, error_name: &str, details: &str) -> Self {
		Self {
			position,
			error_name: error_name.to_owned(),
			details: details.to_owned()
		}
	}

	pub fn as_string(&self) -> String {
		let mut result = String::new();

		if let Some(position) = &self.position {
			result.push_str(format!("In file \"{}\", from position {}:{} to {}:{}\n", position.0.filname(), position.0.line(), position.0.colomn(), position.1.line(), position.1.colomn()).as_str());
		}

		result.push_str(format!("{}: {}", self.error_name, self.details).as_str());

		result
	}
}

pub struct NoFileOrDirError;

impl NoFileOrDirError {
	pub fn new(position: Option<(Position, Position)>, details: &str) -> Error {
		Error::new(position, "No such file or directory", details)
	}
}

pub struct IllegalCharError;

impl IllegalCharError {
	pub fn new(position: Option<(Position, Position)>, details: &str) -> Error {
		Error::new(position, "Illegal character", details)
	}
}