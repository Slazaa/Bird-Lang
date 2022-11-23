use parse::{PatternFunc, Location};

use crate::Node;

use super::Item;

#[derive(Debug, Clone)]
pub struct Items {
	pub items: Vec<Item>,
	pub location: Location
}

pub static ITEMS_PATTERNS: [(&str, &str, PatternFunc<Node>); 3] = [
	("items", "item items", items),
	("items", "item", items),
	("items", "", items)
];

fn items(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Items(Items { items: vec![], location: Location::default() }));
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

	Ok(Node::Items(Items { items: items_vec, location: Location { start: item.location().start, end: items.last().unwrap().location().end } }))
}