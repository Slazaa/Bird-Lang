use parse::PatternFunc;

use crate::Node;

use super::Stmts;

pub static PROGRAM_PATTERNS: [(&str, &str, PatternFunc<Node>); 2] = [
	("program",    "stmts", program),
	("program",    "", program)
];

#[derive(Debug, Clone)]
pub struct Program {
	pub stmts: Option<Stmts>
}

fn program(nodes: &[Node]) -> Result<Node, String> {
	let stmts = if nodes.is_empty() {
		None
	} else {
		match &nodes[0] {
			Node::Stmts(x) => Some(x.to_owned()),
			_ => return Err(format!("Invalid node '{:?}' in 'program'", nodes[0]))
		}
	};

	Ok(Node::Program(Program { stmts }))
}