pub static SEPARATORS: &str = "(){}[]<>,\n";

pub static OPERATORS: [&str; 35] = [
	"+", "-", "*", "/", "%",
	"==", "!=", ">", "<", ">=", "<=",
	"&&", "||", "!",
	"&", "|", "^", "~", "<<", ">>",
	"=", "+=", "-=", "*=", "/=", "%=", "<<=", ">>=", "&=", "^=", "|=",
	":",
	"->",
	"&",
	"::"
];

pub static OPERATOR_CHARS: &str = "+-*/%=!><&|^~:";

pub static KEYWORDS: [&str; 5] = [
	"func"  , "if" , "pub",
	"struct", "var"
];

pub mod compile {
	pub static FUNC_PREFIX: &str = "f_";
}