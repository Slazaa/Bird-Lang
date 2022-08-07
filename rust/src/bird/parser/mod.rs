pub use self::parse::Node;

use super::feedback::Feedback;
use super::lexer::Token;

mod parse;
mod statement;
mod expression;

pub struct Parser;

impl Parser {
	pub fn parse(tokens: &[Token]) -> Result<Node, Feedback> {
		parse::Parser::parse(tokens)
	}
}