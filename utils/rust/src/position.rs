#[derive(Clone, Debug)]
pub struct Position {
	idx: usize,
	line: usize,
	col: usize,
	file_path: Option<String>
}

impl Position {
	pub fn new(idx: usize, line: usize, col: usize, file_path: Option<&str>) -> Self {
		Self {
			idx,
			line,
			col,
			file_path: file_path.map(|x| x.to_owned())
		}
	}

	pub fn idx(&self) -> usize {
		self.idx
	}

	pub fn line(&self) -> usize {
		self.line
	}

	pub fn col(&self) -> usize {
		self.col
	}

	pub fn file_path(&self) -> Option<&String> {
		self.file_path.as_ref()
	}
}