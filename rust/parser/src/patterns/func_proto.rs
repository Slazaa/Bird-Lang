use parse::{PatternFunc, Location, ASTNode};

use crate::Node;

#[derive(Debug, Clone)]
pub struct FuncProto {
	pub id: String,
	pub location: Location
}

pub static FUNC_PROTO_PATTERNS: [(&str, &str, PatternFunc<Node>); 1] = [
	("func_proto", "FUNC ID SEMI", func_proto)
];

fn func_proto(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[1] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[1]))
	};

	Ok(Node::FuncProto(FuncProto { id, location: Location { start: nodes.first().unwrap().token().unwrap().location.start, end: nodes.last().unwrap().token().unwrap().location.end } }))
}