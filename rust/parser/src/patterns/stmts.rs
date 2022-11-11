use parse::PatternFunc;

use crate::Node;

use super::Stmt;

#[derive(Debug, Clone)]
pub struct Stmts {
	pub stmts: Vec<Stmt>
}

pub static STMTS_PATTERNS: [(&str, &str, PatternFunc<Node>); 3] = [
	("stmts", "stmt stmts", stmts),
	("stmts", "stmt", stmts),
	("stmts", "", stmts),
];

fn stmts(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Stmts(Stmts { stmts: vec![] }));
	}

	let node_stmt = match &nodes[0] {
		Node::Stmt(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'stmts'", nodes[0]))
	};

	let node_stmts = match nodes.get(1) {
		Some(Node::Stmts(x)) => x.stmts.clone(),
		_ => Vec::new()
	};

	let mut stmts_vec = vec![node_stmt];
	stmts_vec.extend(node_stmts);

	Ok(Node::Stmts(Stmts { stmts: stmts_vec }))
}