use parse::PatternFunc;

use crate::Node;

use super::Items;

#[derive(Debug, Clone)]
pub struct ExternBlock {
	pub lang: String,
	pub items: Items
}

pub static EXTERN_BLOCK_PATTERNS: [(&str, &str, PatternFunc<Node>); 1] = [
	("extern_block", "EXT STR LCBR items RCBR", extern_block)
];

fn extern_block(nodes: &[Node]) -> Result<Node, String> {
	let lang = match &nodes[1] {
		Node::Token(x) => x.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'extern_block'", nodes[1]))
	};

	let items = match &nodes[3] {
		Node::Items(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'extern_block'", nodes[3]))
	};

	Ok(Node::ExternBlock(ExternBlock { lang, items }))
}