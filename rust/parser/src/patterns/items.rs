use parse::{PatternFunc, Loc};
use bird_utils::*;

use crate::Node;

use super::Item;

#[derive(Debug, Clone)]
pub struct Items {
	pub items: Vec<Item>,
	pub loc: Loc
}

pub static ITEMS_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 3] = [
	("items", "item items", items),
	("items", "item", items),
	("items", "", items)
];

fn items(nodes: &[Node]) -> Result<Node, Feedback> {
	if nodes.is_empty() {
		return Ok(Node::Items(Items { items: vec![], loc: Loc::default() }));
	}

	let item = match &nodes[0] {
		Node::Item(x) => x.to_owned(),
		_ => panic!("If you see this, that means the dev does bad work")
	};

	let items = match nodes.get(1) {
		Some(Node::Items(x)) => x.items.clone(),
		_ => Vec::new()
	};

	let mut items_vec = vec![item.to_owned()];
	items_vec.extend(items.to_owned());

	let mut loc = item.loc().to_owned();
	loc.end = if items.is_empty() {
		item.loc().end.to_owned()
	} else {
		items.last().unwrap().loc().end.to_owned()
	};

	Ok(Node::Items(Items { items: items_vec, loc: loc.to_owned() }))
}