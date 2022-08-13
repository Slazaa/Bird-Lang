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

pub static KEYWORDS: [&str; 7] = [
	"func", "if" , "loop"  ,
	"mut" , "pub", "struct",
	"var"
];

pub mod compile {
	pub static FUNC_PREFIX: &str = "f_";
}