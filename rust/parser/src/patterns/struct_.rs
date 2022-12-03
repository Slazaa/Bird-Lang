use parse::*;
use bird_utils::*;

use crate::Node;

use super::*;

#[derive(Debug, Clone)]
pub struct Struct {
	pub public: Option<bool>,
	pub id: String,
	pub fields: Fields,
	pub loc: Loc
}

pub static STRUCT_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 1] = [
	("struct", "STRCT ID LCBR fields RCBR", struct_)
];

fn struct_(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let fields = match &nodes[3] {
		Node::Fields(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[4].loc().end.to_owned();

	Ok(Node::Struct(Struct { public: None, id, fields, loc }))
}