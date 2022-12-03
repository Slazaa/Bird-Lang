use parse::*;
use bird_utils::*;

use crate::Node;

use super::*;

#[derive(Debug, Clone)]
pub enum Item {
	ConstDecl(ConstDecl),
	Func(Func),
	FuncProto(FuncProto),
	Struct(Struct),
	VarDecl(VarDecl)
}

impl Item {
	pub fn public(&self) -> &Option<bool> {
		match self {
			Self::ConstDecl(x) => &x.public,
			Self::Func(x) => &x.public,
			Self::FuncProto(x) => &x.public,
			Self::Struct(x) => &x.public,
			Self::VarDecl(x) => &x.public
		}
	}

	pub fn public_mut(&mut self) -> &mut Option<bool> {
		match self {
			Self::ConstDecl(x) => &mut x.public,
			Self::Func(x) => &mut x.public,
			Self::FuncProto(x) => &mut x.public,
			Self::Struct(x) => &mut x.public,
			Self::VarDecl(x) => &mut x.public
		}
	}

	pub fn loc(&self) -> &Loc {
		match self {
			Self::ConstDecl(x) => &x.loc,
			Self::Func(x) => &x.loc,
			Self::FuncProto(x) => &x.loc,
			Self::Struct(x) => &x.loc,
			Self::VarDecl(x) => &x.loc
		}
	}
}

pub static ITEM_PATTERNS: [(&str, &str, PatternFunc<Node, Feedback>); 5] = [
	("item", "const_decl", item),
	("item", "func", item),
	("item", "func_proto", item),
	("item", "struct", item),
	("item", "var_decl", item)
];

fn item(nodes: &[Node]) -> Result<Node, Feedback> {
	match &nodes[0] {
		Node::ConstDecl(x) => Ok(Node::Item(Item::ConstDecl(x.to_owned()))),
		Node::Func(x) => Ok(Node::Item(Item::Func(x.to_owned()))),
		Node::FuncProto(x) => Ok(Node::Item(Item::FuncProto(x.to_owned()))),
		Node::Struct(x) => Ok(Node::Item(Item::Struct(x.to_owned()))),
		Node::VarDecl(x) => Ok(Node::Item(Item::VarDecl(x.to_owned()))),
		_ => panic!("If you see this, that means the dev does bad work")
	}
}