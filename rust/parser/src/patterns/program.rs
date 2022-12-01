use parse::{PatternFunc, Loc};
use bird_utils::*;

use crate::Node;

use super::Stmts;

#[derive(Debug, Clone)]
pub struct Program {
	pub stmts: Stmts,
	pub loc: Loc
}

pub static PROGRAM_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 1] = [
	("program", "stmts", program)
];

fn program(nodes: &[Node]) -> Result<Node, Feedback> {
	let stmts = match &nodes[0] {
		Node::Stmts(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let loc = if stmts.stmts.is_empty() {
		Loc::default()
	} else {
		stmts.loc.to_owned()
	};

	Ok(Node::Program(Program { stmts, loc }))
}