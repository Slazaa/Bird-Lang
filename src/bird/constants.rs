pub static SEPARATORS: &str = "(){}[]<>;";

pub static OPERATORS: [&str; 35] = [
	"+", "-", "*", "/", "%",
	"==", "!=", ">", "<", ">=", "<=",
	"&&", "||", "!",
	"&", "|", "^", "~", "<<", ">>",
	"=", "+=", "-=", "*=", "/=", "%=", "<<=", ">>=", "&=", "^=", "|=", ":",
	"->",
	"&",
	"::"
];

pub static OPERATOR_CHARS: &str = "+-*/%=!><&|^~:";

pub static KEYWORDS: [&str; 3] = [
	"func", "pub", "var"
];