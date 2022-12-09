use parse::*;
use bird_utils::*;

use crate::Node;

#[derive(Debug, Clone)]
pub struct Import {
	pub path: String,
	pub loc: Loc
}

pub static IMPORT_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 1] = [
	("import", "IMP STR SEMI", import)
];

pub fn import(nodes: &[Node]) -> Result<Node, Feedback> {
	let path = match &nodes[1] {
		Node::Token(token) if token.name == "STR" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[2].loc().end.to_owned();

	Ok(Node::Import(Import { path, loc }))
}