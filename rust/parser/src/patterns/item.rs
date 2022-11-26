use parse::{PatternFunc, Loc};
use bird_utils::*;

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
	pub fn loc(&self) -> &Loc {
		match self {
			Self::ExternBlock(x) => &x.loc,
			Self::Func(x) => &x.loc,
			Self::FuncProto(x) => &x.loc,
			Self::VarDecl(x) => &x.loc
		}
	}
}

pub static ITEM_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 4] = [
	("item", "extern_block", item),
	("item", "func", item),
	("item", "func_proto", item),
	("item", "var_decl", item)
];

fn item(nodes: &[Node]) -> Result<Node, Feedback> {
	match &nodes[0] {
		Node::ExternBlock(x) => Ok(Node::Item(Item::ExternBlock(x.to_owned()))),
		Node::Func(x) => Ok(Node::Item(Item::Func(x.to_owned()))),
		Node::FuncProto(x) => Ok(Node::Item(Item::FuncProto(x.to_owned()))),
		Node::VarDecl(x) => Ok(Node::Item(Item::VarDecl(x.to_owned()))),
		_ => panic!("If you see this, that means the dev does bad work")
	}
}