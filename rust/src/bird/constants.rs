pub static SEPARATORS: &str = "(){}[],\n";

pub static OPERATORS: [&str; 34] = [
	// Arithmetic
	"+", "-", "*", "/", "%",

	// Relational
	"==", "!=", ">", "<", ">=", "<=",

	// Logical
	"&&", "||", "!",

	// Bitwise
	"&", "|", "^", "~", "<<", ">>",

	// Assignment
	"=", "+=", "-=", "*=", "/=", "%=", "<<=", ">>=", "&=", "^=", "|=",

	// Pointer
	// "&", "*",

	":",
	"->",
	"::",

];

pub static OPERATOR_CHARS: &str = "+-*/%=!><&|^~:";

pub static KEYWORDS: [&str; 17] = [
	"break" , "const" , "continue",
	"else"  , "enum"  , "func"    ,
	"if"    , "impl"  , "loop"    ,
	"match" , "mut"   , "pub"     ,
	"return", "struct", "type"    ,
	"use"   , "var"
];

pub mod compile {
	pub static FUNC_PREFIX: &str = "f_";
	pub static PRIMITIVE_PREFIX: &str = "p_";
}
