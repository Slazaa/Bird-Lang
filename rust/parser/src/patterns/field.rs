use parse::*;
use bird_utils::*;

use crate::Node;

use super::*;

#[derive(Debug, Clone)]
pub struct Field {
	pub id: String,
	pub type_: Type,
	pub loc: Loc
}

pub static FIELD_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 1] = [
	("field", "ID COL type", field),
];

fn field(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[0] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let type_ = match &nodes[2] {
		Node::Type(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[2].loc().end.to_owned();

	Ok(Node::Field(Field { id, type_, loc }))
}