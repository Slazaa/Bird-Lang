use parse::{PatternFunc, Loc};

use crate::Node;

use super::Stmt;

#[derive(Debug, Clone)]
pub struct Stmts {
	pub stmts: Vec<Stmt>,
	pub loc: Loc
}

pub static STMTS_PATTERNS: [(&str, &str, PatternFunc<Node>); 3] = [
	("stmts", "stmt stmts", stmts),
	("stmts", "stmt", stmts),
	("stmts", "", stmts),
];

fn stmts(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Stmts(Stmts { stmts: vec![], loc: Loc::default() }));
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

	let mut loc = stmt.loc();
	loc.end = if stmts.is_empty() {
		stmt.loc().end
	} else {
		stmts.last().unwrap().loc().end
	};

	Ok(Node::Stmts(Stmts { stmts: stmts_vec, loc }))
}