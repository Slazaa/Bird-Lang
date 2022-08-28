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

pub static KEYWORDS: [&str; 17] = [
	"as"    , "break"   , "case"  ,
	"const" , "continue", "else"  ,
	"enum"  , "func"    , "if"    ,
	"import", "loop"    , "mut"   ,
	"pub"   , "return"  , "struct",
	"switch", "var"
];

pub mod compile {
	pub static FUNC_PREFIX: &str = "f_";
}