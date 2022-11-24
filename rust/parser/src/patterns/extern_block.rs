use parse::{PatternFunc, Loc, ASTNode};

use crate::Node;

use super::Items;

#[derive(Debug, Clone)]
pub struct ExternBlock {
	pub lang: String,
	pub items: Items,
	pub loc: Loc
}

pub static EXTERN_BLOCK_PATTERNS: [(&str, &str, PatternFunc<Node>); 1] = [
	("extern_block", "EXT STR LCBR items RCBR", extern_block)
];

fn extern_block(nodes: &[Node]) -> Result<Node, String> {
	let lang = match &nodes[1] {
		Node::Token(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'extern_block'", nodes[1]))
	};

	let items = match &nodes[3] {
		Node::Items(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'extern_block'", nodes[3]))
	};

	let mut loc = nodes[0].token().unwrap().loc.to_owned();
	loc.end = nodes[4].token().unwrap().loc.end.to_owned();

	Ok(Node::ExternBlock(ExternBlock { lang: lang.symbol, items, loc }))
}