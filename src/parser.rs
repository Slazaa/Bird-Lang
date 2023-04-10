use nom::Err;
use nom_supreme::error::ErrorTree;

use exprs::file::File;

pub mod exprs;

pub fn parse_file(input: &str) -> Result<File, Err<ErrorTree<&str>>> {
	match File::parse(input) {
		Ok((_, x)) => Ok(x),
		Err(e) => Err(e)
	}
}