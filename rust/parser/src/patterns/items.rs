use parse::{PatternFunc, Loc};

use crate::Node;

use super::Item;

#[derive(Debug, Clone)]
pub struct Items {
	pub items: Vec<Item>,
	pub loc: Loc
}

pub static ITEMS_PATTERNS: [(&str, &str, PatternFunc<Node>); 3] = [
	("items", "item items", items),
	("items", "item", items),
	("items", "", items)
];

fn items(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Items(Items { items: vec![], loc: Loc::default() }));
	}

	let item = match &nodes[0] {
		Node::Item(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'items'", nodes[0]))
	};

	let items = match nodes.get(1) {
		Some(Node::Items(x)) => x.items.clone(),
		_ => Vec::new()
	};

	let mut items_vec = vec![item.to_owned()];
	items_vec.extend(items.to_owned());

	let mut loc = item.loc();
	loc.end = if items.is_empty() {
		item.loc().end
	} else {
		items.last().unwrap().loc().end
	};

	Ok(Node::Items(Items { items: items_vec, loc }))
}