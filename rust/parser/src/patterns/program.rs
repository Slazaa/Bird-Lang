use parse::{PatternFunc, Location};

use crate::Node;

use super::Stmts;

#[derive(Debug, Clone)]
pub struct Program {
	pub stmts: Stmts,
	pub location: Location
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

	let location = if stmts.stmts.is_empty() {
		Location::default()
	} else {
		stmts.location
	};

	Ok(Node::Program(Program { stmts, location }))
}