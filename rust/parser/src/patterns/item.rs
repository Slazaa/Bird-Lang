use parse::PatternFunc;

use crate::Node;

use super::{VarDecl, FuncProto, Func};

#[derive(Debug, Clone)]
pub enum Item {
	Func(Func),
	FuncProto(FuncProto),
	VarDecl(VarDecl)
}

pub static ITEM_PATTERNS: [(&str, &str, PatternFunc<Node>); 3] = [
	("item", "func", item),
	("item", "func_proto", item),
	("item", "var_decl", item)
];

fn item(nodes: &[Node]) -> Result<Node, String> {
	match &nodes[0] {
		Node::Func(x) => Ok(Node::Item(Item::Func(x.to_owned()))),
		Node::FuncProto(x) => Ok(Node::Item(Item::FuncProto(x.to_owned()))),
		Node::VarDecl(x) => Ok(Node::Item(Item::VarDecl(x.to_owned()))),
		_ => Err(format!("Invalid node '{:?}' in 'item'", nodes[0]))
	}
}