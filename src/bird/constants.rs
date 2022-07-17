pub static SEPARATORS: [&str; 9] = [
	"(", ")", "{", "}",
	"[", "]", "<", ">",
	";"
];

pub static SEPARATOR_CHARS: &str = "(){}[]<>;";

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