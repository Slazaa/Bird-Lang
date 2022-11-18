use parse::PatternFunc;

use crate::Node;

use super::Item;

#[derive(Debug, Clone)]
pub struct Items {
	pub items: Vec<Item>
}

pub static ITEMS_PATTERNS: [(&str, &str, PatternFunc<Node>); 3] = [
	("items", "item items", items),
	("items", "item", items),
	("items", "", items)
];

fn items(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Items(Items { items: vec![] }));
	}

	let node_item = match &nodes[0] {
		Node::Item(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'items'", nodes[0]))
	};

	let node_items = match nodes.get(1) {
		Some(Node::Items(x)) => x.items.clone(),
		_ => Vec::new()
	};

	let mut items_vec = vec![node_item];
	items_vec.extend(node_items);

	Ok(Node::Items(Items { items: items_vec }))
}