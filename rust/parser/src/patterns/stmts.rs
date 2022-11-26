use parse::{PatternFunc, Loc};
use bird_utils::*;

use crate::Node;

use super::Stmt;

#[derive(Debug, Clone)]
pub struct Stmts {
	pub stmts: Vec<Stmt>,
	pub loc: Loc
}

pub static STMTS_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 3] = [
	("stmts", "stmt stmts", stmts),
	("stmts", "stmt", stmts),
	("stmts", "", stmts),
];

fn stmts(nodes: &[Node]) -> Result<Node, Feedback> {
	if nodes.is_empty() {
		return Ok(Node::Stmts(Stmts { stmts: vec![], loc: Loc::default() }));
	}

	let stmt = match &nodes[0] {
		Node::Stmt(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let stmts = match nodes.get(1) {
		Some(Node::Stmts(x)) => x.stmts.clone(),
		_ => Vec::new()
	};

	let mut stmts_vec = vec![stmt.to_owned()];
	stmts_vec.extend(stmts.to_owned());

	let mut loc = stmt.loc().to_owned();
	loc.end = if stmts.is_empty() {
		stmt.loc().end.to_owned()
	} else {
		stmts.last().unwrap().loc().end.to_owned()
	};

	Ok(Node::Stmts(Stmts { stmts: stmts_vec, loc: loc.to_owned() }))
}