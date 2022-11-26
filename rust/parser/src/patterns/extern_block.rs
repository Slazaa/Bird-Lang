use parse::{PatternFunc, Loc, ASTNode};
use bird_utils::*;

use crate::Node;

use super::Stmts;

#[derive(Debug, Clone)]
pub struct ExternBlock {
	pub lang: String,
	pub stmts: Stmts,
	pub loc: Loc
}

pub static EXTERN_BLOCK_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 1] = [
	("extern_block", "EXT STR LCBR stmts RCBR", extern_block)
];

fn extern_block(nodes: &[Node]) -> Result<Node, Feedback> {
	let lang = match &nodes[1] {
		Node::Token(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let stmts = match &nodes[3] {
		Node::Stmts(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[4].token().unwrap().loc.end.to_owned();

	Ok(Node::ExternBlock(ExternBlock { lang: lang.symbol, stmts, loc }))
}