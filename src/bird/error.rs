pub struct Error {
	error_name: String,
	details: String
}

impl Error {
	pub fn new(error_name: &str, details: &str) -> Self {
		Self {
			error_name: error_name.to_owned(),
			details: details.to_owned()
		}
	}

	pub fn as_string(&self) -> String {
		format!("{}: {}", self.error_name, self.details)
	}
}

pub struct NoFileOrDirError;

impl NoFileOrDirError {
	pub fn new(details: &str) -> Error {
		Error::new("No such file or directory", details)
	}
}

pub struct IllegalCharError;

impl IllegalCharError {
	pub fn new(details: &str) -> Error {
		Error::new("Illegal character", details)
	}
}