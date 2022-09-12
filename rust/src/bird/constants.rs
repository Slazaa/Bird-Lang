pub static SEPARATORS: &str = "(){}[]<>,\n";

pub static OPERATORS: [&str; 35] = [
    "+", "-", "*", "/", "%", "==", "!=", ">", "<", ">=", "<=", "&&", "||", "!", "&", "|", "^", "~",
    "<<", ">>", "=", "+=", "-=", "*=", "/=", "%=", "<<=", ">>=", "&=", "^=", "|=", ":", "->", "&",
    "::",
];

pub static OPERATOR_CHARS: &str = "+-*/%=!><&|^~:";

pub static KEYWORDS: [&str; 18] = [
    "as", "break", "case", "const", "continue", "else", "enum", "func", "if", "import", "loop",
    "match", "mut", "pub", "return", "struct", "type", "var",
];

pub mod compile {
    pub static FUNC_PREFIX: &str = "f_";
    pub static PRIMITIVE_PREFIX: &str = "p_";
}
