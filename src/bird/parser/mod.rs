pub use self::parser::Node;

use super::feedback::Feedback;
use super::lexer::Token;

mod parser;
mod statement;
mod expression;

pub struct Parser;

impl Parser {
	pub fn parse(tokens: &[Token]) -> Result<Node, Feedback> {
		parser::Parser::parse(tokens)
	}
}