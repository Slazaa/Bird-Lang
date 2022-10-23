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