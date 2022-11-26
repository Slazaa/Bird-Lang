use parse::{PatternFunc, Loc, ASTNode};
use bird_utils::*;

use crate::Node;

#[derive(Debug, Clone)]
pub struct FuncProto {
	pub id: String,
	pub loc: Loc
}

pub static FUNC_PROTO_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 1] = [
	("func_proto", "FUNC ID SEMI", func_proto)
];

fn func_proto(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[2].token().unwrap().loc.end.to_owned();

	Ok(Node::FuncProto(FuncProto { id, loc }))
}