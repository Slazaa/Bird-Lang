use parse::{PatternFunc, Location};

use crate::Node;

use super::Stmt;

#[derive(Debug, Clone)]
pub struct Stmts {
	pub stmts: Vec<Stmt>,
	pub location: Location
}

pub static STMTS_PATTERNS: [(&str, &str, PatternFunc<Node>); 3] = [
	("stmts", "stmt stmts", stmts),
	("stmts", "stmt", stmts),
	("stmts", "", stmts),
];

fn stmts(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Stmts(Stmts { stmts: vec![], location: Location::default() }));
	}

	let stmt = match &nodes[0] {
		Node::Stmt(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'stmts'", nodes[0]))
	};

	let stmts = match nodes.get(1) {
		Some(Node::Stmts(x)) => x.stmts.clone(),
		_ => Vec::new()
	};

	let mut stmts_vec = vec![stmt.to_owned()];
	stmts_vec.extend(stmts.to_owned());

	Ok(Node::Stmts(Stmts { stmts: stmts_vec, location: Location { start: stmt.location().start, end: stmts.last().unwrap().location().end } }))
}