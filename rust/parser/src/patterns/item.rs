use parse::{PatternFunc, Location};

use crate::Node;

use super::{VarDecl, FuncProto, Func, ExternBlock};

#[derive(Debug, Clone)]
pub enum Item {
	ExternBlock(ExternBlock),
	Func(Func),
	FuncProto(FuncProto),
	VarDecl(VarDecl)
}

impl Item {
	pub fn location(&self) -> Location {
		match self {
			Self::ExternBlock(x) => x.location,
			Self::Func(x) => x.location,
			Self::FuncProto(x) => x.location,
			Self::VarDecl(x) => x.location
		}
	}
}

pub static ITEM_PATTERNS: [(&str, &str, PatternFunc<Node>); 4] = [
	("item", "extern_block", item),
	("item", "func", item),
	("item", "func_proto", item),
	("item", "var_decl", item)
];

fn item(nodes: &[Node]) -> Result<Node, String> {
	match &nodes[0] {
		Node::ExternBlock(x) => Ok(Node::Item(Item::ExternBlock(x.to_owned()))),
		Node::Func(x) => Ok(Node::Item(Item::Func(x.to_owned()))),
		Node::FuncProto(x) => Ok(Node::Item(Item::FuncProto(x.to_owned()))),
		Node::VarDecl(x) => Ok(Node::Item(Item::VarDecl(x.to_owned()))),
		_ => Err(format!("Invalid node '{:?}' in 'item'", nodes[0]))
	}
}