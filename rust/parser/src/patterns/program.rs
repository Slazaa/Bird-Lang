use parse::{PatternFunc, Loc};

use crate::Node;

use super::Stmts;

#[derive(Debug, Clone)]
pub struct Program {
	pub stmts: Stmts,
	pub loc: Loc
}

pub static PROGRAM_PATTERNS: [(&str, &str, PatternFunc<Node>); 2] = [
	("program", "stmts", program),
	("program", "", program)
];

fn program(nodes: &[Node]) -> Result<Node, String> {
	let stmts = match &nodes[0] {
		Node::Stmts(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'program'", nodes[0]))
	};

	let loc = if stmts.stmts.is_empty() {
		Loc::default()
	} else {
		stmts.loc.to_owned()
	};

	Ok(Node::Program(Program { stmts, loc }))
}