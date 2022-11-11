use parse::PatternFunc;

use crate::Node;

use super::Stmts;

pub static PROGRAM_PATTERNS: [(&str, &str, PatternFunc<Node>); 2] = [
	("program",    "stmts", program),
	("program",    "", program)
];

fn program(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Program(Stmts { stmts: vec![] }));
	}

	match &nodes[0] {
		Node::Stmts(x) => Ok(Node::Program(x.to_owned())),
		_ => Err(format!("Invalid node '{:?}' in 'program'", nodes[0]))
	}
}