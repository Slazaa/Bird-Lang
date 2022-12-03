use parse::*;
use bird_utils::*;

use crate::Node;

#[derive(Debug, Clone)]
pub struct FuncCall {
	pub id: String,
	pub loc: Loc
}

pub static FUNC_CALL_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 1] = [
	("func_call", "ID LPAR RPAR", func_call)
];

fn func_call(nodes: &[Node]) -> Result<Node, Feedback> {
	let id = match &nodes[0] {
		Node::Token(token) if token.name == "ID" => token.symbol.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].loc().to_owned();
	loc.end = nodes[2].loc().end.to_owned();

	Ok(Node::FuncCall(FuncCall { id, loc }))
}