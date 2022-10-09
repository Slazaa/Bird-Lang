/*
use crate::bird::feedback::*;
use crate::bird::lexer::{Position, Token, TokenType};

/// This enum defines the nodes of the AST.
#[derive(Clone, Debug)]
pub enum Node {
	Undefined,
	// ----------
	Program(Box<Node>),
	// ----------
	Literal(String, (Position, Position)),
	Identifier(String, (Position, Position)),
	Operator(String, (Position, Position)),
	Block(Vec<Node>),
	// ----------
	Label { identifier: Box<Node>, holding: Box<Node> },
	// Expressions
	UnaryExpr { operator: Box<Node>, node: Box<Node>, wrapped: bool },
	BinExpr { operator: Box<Node>, left: Box<Node>, right: Box<Node>, wrapped: bool },
	// ----------
	Field { identifier: Box<Node>, field_type: Box<Node>},
	// ----------
	FuncProto { public: bool, generics: Vec<Node>, params: Vec<Node>, return_type: Box<Option<Node>> },
	// Items
	FuncItem { proto: Box<Node>, body: Box<Node> },
	VarItem { public: bool, value: Box<Option<Node>> },
	TypeItem { public: bool, value: Box<Node> },
	StructItem { public: bool, fields: Vec<Node> },
	//EnumItem { public: bool, identifier: Box<Node>, values: Vec<Node> },
	// ----------
	Assignment { destination: Box<Node>, operator: Box<Node>, value: Box<Node> },
	// ----------
	FuncCall { identifier: Box<Node>, params: Vec<Node> },
	// Statements
	IfStmt { condition: Box<Node>, body: Box<Node> },
	LoopStmt { condition: Box<Node>, body: Box<Node> },
	ReturnStmt { expr: Box<Node> },
	// Types
	Type { identifier: Box<Node> },
	TypePtr { hodling_type: Box<Node>, mutable: bool }
}

impl Node {
	pub fn literal(value: &str, pos: (&Position, &Position)) -> Self {
		let pos = (pos.0.clone(), pos.1.clone());
		Self::Literal(value.to_owned(), pos)
	}

	pub fn identifier(value: &str, pos: (&Position, &Position)) -> Self {
		let pos = (pos.0.clone(), pos.1.clone());
		Self::Identifier(value.to_owned(), pos)
	}

	pub fn operator(value: &str, pos: (&Position, &Position)) -> Self {
		let pos = (pos.0.clone(), pos.1.clone());
		Self::Operator(value.to_owned(), pos)
	}

	pub fn block(nodes: Vec<Node>) -> Self {
		Self::Block(nodes)
	}
}

/// The `Parser` generates an AST from a `Token` list.
pub struct Parser {
	tokens: Vec<Token>,
	token_index: usize,
	current_token: Token,
	parent_node: Node,
	next_pub: Option<Token>,
	scoped_ids: Vec<Vec<String>>
}

impl Parser {
	/// Parse the `Token` list into an AST.
	pub fn parse(tokens: &[Token]) -> Result<Node, Feedback> {
		let current_token = match tokens.first() {
			Some(x) => x.clone(),
			None => return Ok(Node::Program(Box::new(Node::block(vec![]))))
		};

		let mut parser = Self {
			tokens: tokens.to_vec(),
			token_index: 0,
			current_token,
			parent_node: Node::Program(Box::new(Node::block(vec![]))),
			next_pub: None,
			scoped_ids: Vec::new()
		};

		let block = parser.block()?;

		if let Node::Program(body) = &mut parser.parent_node {
			*body = Box::new(block);
		}

		Ok(parser.parent_node)
	}

    pub fn lex_and_parse(src: &str) -> Result<Node, Feedback> {
        let tokens = lexer::parse(src)?;
        Ok(Self::parse(tokens)?)
    }

	/// Returns true if there is more token, else returns false
	fn is_more_token(&self) -> bool {
		self.token_index < self.tokens.len()
	}

	/// Returns reference to the current token.
	fn current_token(&self) -> &Token {
		&self.current_token
	}

	/// Returns a reference the parent node.
	fn parent_node(&self) -> &Node {
		&self.parent_node
	}

	/// Returns a mutable reference to the parent node.
	fn parent_node_mut(&mut self) -> &mut Node {
		&mut self.parent_node
	}

	/// Returns an option to a reference to the `pub` token.
	/// If it is `Some`, the next token is affected by the `pub` keyword.
	/// Else returns `None`.
	fn next_pub(&self) -> Option<&Token> {
		self.next_pub.as_ref()
	}

	/// Returns an option to a mutable reference to next pub.
	fn next_pub_mut(&mut self) -> &mut Option<Token> {
		&mut self.next_pub
	}

	/// Advances to the next `Token`.
	/// Returns a reference to the current `Node`.
	fn advance(&mut self, expected: Option<&str>) -> Result<(), Option<Feedback>> {
		self.token_index += 1;

		if self.token_index < self.tokens.len() {
			self.current_token = self.tokens[self.token_index].clone();
			return Ok(());
		}

		Err(expected.map(|e| Error::expected(self.current_token().pos(), e, None)))
	}

	fn cancel_advance(&mut self) -> Result<(), Feedback> {
		if self.token_index.checked_sub(1).is_some() {
			return Err(Error::unspecified("Index overflow"));
		}

		self.current_token = self.tokens[self.token_index].clone();
		Ok(())
	}

	/// Advances the current `Token` util it is not a new line `Token`.
	fn skip_new_lines(&mut self) {
		if !self.is_more_token() {
			return;
		}

		loop {
			match self.current_token().token_type() {
				TokenType::Separator if self.current_token().symbol() == "\n" => (),
				_ => break
			};

			if self.advance(None).is_err() {
				break;
			}
		}
	}

	/// Check if the current token is in the right scope.
	fn check_scope(&self) -> bool {
		match self.parent_node() {
			Node::Program { .. } => match self.current_token().token_type() {
				TokenType::Keyword => matches!(
					self.current_token().symbol(),
					"const"  | "enum" | "func" |
					"struct" | "type" | "var"
				),
				_ => false
			}
			_ => match self.current_token().token_type() {
				TokenType::Literal | TokenType::Identifier | TokenType::Operator => true,
				TokenType::Separator => matches!(self.current_token().symbol(), "("),
				TokenType::Keyword => matches!(
					self.current_token().symbol(),
					"break" | "continue" | "const"  |
					"else"  | "enum"     | "func"   |
					"if"    | "loop"     | "return" |
					"type"  | "var"
				)
			}
		}
	}

	/// Evaluate a sequence of token and create the corresponding node.
	fn eval(&mut self) -> Result<Node, Feedback> {
		if let Some(next_pub) = self.next_pub() {
			match self.current_token().symbol() {
				"func" => (),
				_ => return Err(Error::unexpected(next_pub.pos(), &format!("'{}'", self.current_token().symbol())))
			}
		}

		if !self.check_scope() {
			return Err(Error::unexpected(self.current_token().pos(), &format!("'{}'", self.current_token().symbol())));
		}

		match self.current_token().token_type() {
			TokenType::Literal => self.bin_expr(),
			TokenType::Identifier | TokenType::Operator => self.label(),
			TokenType::Separator => match self.current_token().symbol() {
				"(" => self.assignment(),
				_ => todo!()
			}
			TokenType::Keyword => match self.current_token().symbol() {
				"func"   => self.func_item(),
				"if"     => self.if_stmt(),
				"loop"   => self.loop_stmt(),
				"return" => self.return_stmt(),
				"struct" => self.struct_item(),
				"type"   => self.type_item(),
				"var"    => self.var_item(),
				_        => todo!()
			}
		}
	}

	fn literal(&mut self) -> Result<Node, Feedback> {
		let res = match self.current_token().token_type() {
			TokenType::Literal => Node::literal(self.current_token().symbol(), self.current_token().pos()),
			_ => return Err(Error::expected(self.current_token().pos(), "literal", Some(&format!("'{}'", self.current_token().symbol()))))
		};

		self.advance(None).unwrap_or(());
		Ok(res)
	}

	fn identifier(&mut self) -> Result<Node, Feedback> {
		let res = match self.current_token().token_type() {
			TokenType::Identifier => Node::identifier(self.current_token().symbol(), self.current_token().pos()),
			_ => return Err(Error::expected(self.current_token().pos(), "identifier", Some(&format!("'{}'", self.current_token().symbol()))))
		};

		self.advance(None).unwrap_or(());
		Ok(res)
	}

	fn operator(&mut self) -> Result<Node, Feedback> {
		let res = match self.current_token().token_type() {
			TokenType::Operator => Node::operator(self.current_token().symbol(), self.current_token().pos()),
			_ => return Err(Error::expected(self.current_token().pos(), "operator", Some(&format!("'{}'", self.current_token().symbol()))))
		};

		self.advance(None).unwrap_or(());
		Ok(res)
	}

	fn operation_priority(&mut self) -> Result<Node, Feedback> {
		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == "(" => (),
			_ => return Err(Error::expected(self.current_token().pos(), "'('", Some(&format!("'{}'", self.current_token().symbol()))))
		}

		if let Err(Some(feedback)) = self.advance(Some("expression")) {
			return Err(feedback);
		}

		let mut res = self.bin_expr()?;

		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == ")" => (),
			_ => return Err(Error::expected(self.current_token().pos(), "')'", Some(&format!("'{}'", self.current_token().symbol()))))
		}

		self.advance(None).unwrap_or(());

		match &mut res {
			Node::BinExpr { wrapped, .. } | Node::UnaryExpr { wrapped, .. } => *wrapped = true,
			_ => ()
		}

		Ok(res)
	}

	fn block(&mut self) -> Result<Node, Feedback> {
		self.scoped_ids.push(Vec::new());

		let mut statements = vec![];

		loop {
			self.skip_new_lines();

			if !self.is_more_token() {
				break;
			}

			match &self.parent_node {
				Node::Program { .. } => (),
				_ if self.current_token().symbol() == "}" => break,
				_ => ()
			}

			match self.current_token().token_type() {
				TokenType::Keyword if self.current_token().symbol() == "pub" => {
					self.next_pub = Some(self.current_token().clone());

					if let Err(Some(feedback)) = self.advance(Some("item")) {
						return Err(feedback);
					}

					continue;
				}
				_ => ()
			}

			let statement = self.eval()?;

			if let Some(next_pub) = &self.next_pub {
				return Err(Error::expected(next_pub.pos(), "item", None));
			}

			statements.push(statement);
		}

		self.scoped_ids.pop();

		Ok(Node::Block(statements))
	}

	fn label(&mut self) -> Result<Node, Feedback> {
		let identifier = self.identifier()?;

		self.advance(None).unwrap_or(());

		if self.current_token().symbol() != ":" {
			self.cancel_advance()?;
			return self.assignment();
		}

		Ok(Node::Label { identifier: Box::new(identifier), holding: Box::new(self.eval()?) })
	}

	fn bin_expr(&mut self) -> Result<Node, Feedback> {
		let left = match self.current_token().token_type() {
			TokenType::Literal => self.literal()?,
			TokenType::Identifier => self.func_call()?,
			_ => self.eval()?
		};

		let operator = match self.current_token().token_type() {
			TokenType::Operator => self.operator()?,
			_ => return Ok(left)
		};

		Ok(Node::BinExpr { operator: Box::new(operator), left: Box::new(left), right: Box::new(self.bin_expr()?), wrapped: false })
	}

	fn unary_expr(&mut self) -> Result<Node, Feedback> {
		match self.current_token().symbol() {
			"+" | "-" | "!" | "*" => (),
			_ => return Err(Error::expected(self.current_token().pos(), "'+', '-', '!' or '*'", Some(&format!("'{}'", self.current_token().symbol()))))
		}

		Ok(Node::UnaryExpr { operator: Box::new(self.operator()?), node: Box::new(self.eval()?), wrapped: false })
	}

	fn field(&mut self) -> Result<Node, Feedback> {
		let identifier = self.identifier()?;

		match self.current_token().token_type() {
			TokenType::Operator if self.current_token().symbol() == ":" => (),
			_ => return Err(Error::expected(self.current_token().pos(), "':'", Some(&format!("'{}'", self.current_token().symbol()))))
		};

		if let Err(Some(feedback)) = self.advance(Some("type")) {
			return Err(feedback);
		}

		Ok(Node::Field { identifier: Box::new(identifier), field_type: Box::new(self.type_node()?) })
	}
	
	fn func_proto(&mut self) -> Result<Node, Feedback> {
		let public = match self.next_pub() {
			Some(_) => {
				*self.next_pub_mut() = None;
				true
			}
			None => false
		};

		let mut func_proto = Node::FuncProto { public, generics: vec![], params: vec![], return_type: Box::new(None) };

		if self.advance(None).is_err() {
			return Ok(func_proto);
		}

		self.skip_new_lines();

		if self.current_token().symbol() == "<" {
			loop {
				self.skip_new_lines();

				if let Node::FuncProto { generics, .. } = &mut func_proto {
					generics.push(self.identifier()?);
				}

				match self.current_token().symbol() {
					"," => {
						if let Err(Some(feedback)) = self.advance(Some("identifier")) {
							return Err(feedback);
						}
					}
					">" => break,
					_ => return Err(Error::expected(self.current_token().pos(), "',' or '>'", Some(&format!("'{}'", self.current_token().symbol()))))
				}
			}
		}

		self.skip_new_lines();

		if self.advance(None).is_err() {
			return Ok(func_proto);
		}

		self.skip_new_lines();

		if self.current_token().symbol() == "(" {
			loop {
				self.skip_new_lines();

				if let Node::FuncProto { params, .. } = &mut func_proto {
					params.push(self.field()?);
				}

				if let Err(Some(feedback)) = self.advance(Some("',' or ')'")) {
					return Err(feedback);
				}

				self.skip_new_lines();

				match self.current_token().symbol() {
					"," => {
						if let Err(Some(feedback)) = self.advance(Some("identifier")) {
							return Err(feedback);
						}
					}
					")" => break,
					_ => return Err(Error::expected(self.current_token().pos(), "',' or ')'", Some(&format!("'{}'", self.current_token().symbol()))))
				};
			}
		}

		self.skip_new_lines();

		if self.advance(None).is_err() {
			return Ok(func_proto);
		}

		self.skip_new_lines();

		if self.current_token().symbol() == "->" {
			if let Node::FuncProto { return_type, .. } = &mut func_proto {
				if let Err(Some(feedback)) = self.advance(Some("type")) {
					return Err(feedback);
				}

				*return_type = Box::new(Some(self.type_node()?));
			}

			self.advance(None).unwrap_or(());
		}

		Ok(func_proto)
	}

	fn func_item(&mut self) -> Result<Node, Feedback> {
		let func_proto = self.func_proto()?;
		let mut func_item = Node::FuncItem { proto: Box::new(func_proto.clone()), body: Box::new(Node::block(vec![])) };

		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == "{" => {
				if let Err(Some(feedback)) = self.advance(Some("statement or '}'")) {
					return Err(feedback);
				}

				let parent_node = self.parent_node().clone();

				*self.parent_node_mut() = func_item.clone();
				
				if let Node::FuncItem { body, .. } = &mut func_item {
					*body = Box::new(self.block()?);
				}

				*self.parent_node_mut() = parent_node;

				match self.current_token().token_type() {
					TokenType::Separator if self.current_token().symbol() == "}" => (),
					_ => return Err(Error::expected(self.current_token().pos(), "'}'", Some(&format!("'{}'", self.current_token().symbol()))))
				}
			}
			_ => return Ok(func_proto)
		}

		self.advance(None).unwrap_or(());
		Ok(func_item)
	}

	fn return_stmt(&mut self) -> Result<Node, Feedback> {
		if let Err(Some(feedback)) = self.advance(Some("expression")) {
			return Err(feedback);
		}

		Ok(Node::ReturnStmt { expr: Box::new(self.eval()?) })
	}

	fn struct_item(&mut self) -> Result<Node, Feedback> {
		let public = match self.next_pub() {
			Some(_) => {
				*self.next_pub_mut() = None;
				true
			}
			None => false
		};

		let mut struct_item = Node::StructItem { public, fields: vec![] };

		if self.advance(None).is_err() {
			return Ok(struct_item);
		}

		self.skip_new_lines();

		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == "{" => {
				loop {
					if let Err(Some(feedback)) = self.advance(Some("field")) {
						return Err(feedback);
					}

					self.skip_new_lines();

					if let Node::StructItem { fields, .. } = &mut struct_item {
						fields.push(self.field()?);
					}

					if let Err(Some(feedback)) = self.advance(Some("new line or '}'")) {
						return Err(feedback);
					}

					match self.current_token().symbol() {
						"\n" => self.skip_new_lines(),
						"}" => break,
						_ => return Err(Error::expected(self.current_token().pos(), "new line or '}'", Some(&format!("'{}'", self.current_token().symbol()))))
					};
				}
			}
			TokenType::Separator if self.current_token().symbol() == "(" => todo!(),
			_ => todo!()
		}

		self.advance(None).unwrap_or(());
		Ok(struct_item)
	}

	fn type_item(&mut self) -> Result<Node, Feedback> {
		let public = match self.next_pub() {
			Some(_) => {
				*self.next_pub_mut() = None;
				true
			}
			None => false
		};

		if let Err(Some(feedback)) = self.advance(Some("=")) {
			return Err(feedback);
		}

		match self.current_token().token_type() {
			TokenType::Operator if self.current_token().symbol() == "=" => (),
			_ => return Err(Error::expected(self.current_token().pos(), "'='", Some(&format!("'{}'", self.current_token().symbol()))))
		};

		if let Err(Some(feedback)) = self.advance(Some("type")) {
			return Err(feedback);
		}

		let value = self.type_node()?;

		self.advance(None).unwrap_or(());
		Ok(Node::TypeItem { public, value: Box::new(value) })
	}

	fn var_item(&mut self) -> Result<Node, Feedback> {
		let mut public = false;

		if let Node::Program { .. } = self.parent_node() {
			if self.next_pub().is_some() {
				*self.next_pub_mut() = None;
				public = true;
			}
		}

		let mut var_item = Node::VarItem { public, value: Box::new(None) };

		if self.advance(None).is_err() {
			return Ok(var_item);
		}

		if self.current_token().symbol() == "=" {
			if let Err(Some(feedback)) = self.advance(Some("expression")) {
				return Err(feedback);
			}

			if let Node::VarItem { value, .. } = &mut var_item {
				*value = Box::new(Some(self.bin_expr()?));
			}
		}

		self.advance(None).unwrap_or(());
		Ok(var_item)
	}

	fn assignment(&mut self) -> Result<Node, Feedback> {
		let destination = match self.current_token().token_type() {
			TokenType::Identifier => self.bin_expr()?,
			TokenType::Operator => self.unary_expr()?,
			TokenType::Separator => self.operation_priority()?,
			_ => return Err(Error::expected(self.current_token().pos(), "'identifier", Some(&format!("'{}'", self.current_token().symbol()))))
		};

		match self.current_token().symbol() {
			"=" | "+=" | "-=" | "*=" | "/=" | "%=" | "<<=" | ">>=" | "&=" | "^=" | "|=" => (),
			_ => return Ok(destination)
		}

		Ok(Node::Assignment { destination: Box::new(destination), operator: Box::new(self.operator()?), value: Box::new(self.eval()?) })
	}

	fn func_call(&mut self) -> Result<Node, Feedback> {
		let identifier = self.identifier()?;

		match self.current_token().symbol() {
			"(" => (),
			_ => return Ok(identifier)
		}

		if let Err(Some(feedback)) = self.advance(Some("parameters or ')'")) {
			return Err(feedback);
		}

		let mut params = Vec::new();

		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == ")" => (),
			_ => {
				loop {
					self.skip_new_lines();
					params.push(self.eval()?);

					match self.current_token().symbol() {
						"," => {
							if let Err(Some(feedback)) = self.advance(Some("parameter or ')'")) {
								return Err(feedback);
							}
						}
						")" => break,
						_ => return Err(Error::expected(self.current_token().pos(), "',' or ')'", Some(&format!("'{}'", self.current_token().symbol()))))
					};
				}
			}
		}

		self.advance(None).unwrap_or(());
		Ok(Node::FuncCall { identifier: Box::new(identifier), params })
	}

	fn control_flow(&mut self, node: &mut Node) -> Result<(), Feedback> {
		if let Err(Some(feedback)) = self.advance(Some("expression")) {
			return Err(feedback);
		}

		match node {
			Node::IfStmt { condition, .. } => *condition = Box::new(self.eval()?),
			Node::LoopStmt { condition, .. } => *condition = Box::new(self.eval()?),
			_ => return Err(Error::unspecified("Invalid node"))
		}

		if let Err(Some(feedback)) = self.advance(Some("'{'")) {
			return Err(feedback);
		}

		self.skip_new_lines();

		if self.current_token().symbol() != "{" {
			return Err(Error::expected(self.current_token().pos(), "{", Some(&format!("'{}'", self.current_token().symbol()))));
		}

		if let Err(Some(feedback)) = self.advance(Some("statements or '}'")) {
			return Err(feedback);
		}

		let parent_node = self.parent_node().clone();

		*self.parent_node_mut() = node.clone();
		let block = self.block()?;
		*self.parent_node_mut() = parent_node;

		match node {
			Node::IfStmt { body, .. } => *body = Box::new(block),
			Node::LoopStmt { body, .. } => *body = Box::new(block),
			_ => return Err(Error::unspecified("Invalid node"))
		}

		match self.current_token().token_type() {
			TokenType::Separator if self.current_token().symbol() == "}" => (),
			_ => return Err(Error::expected(self.current_token().pos(), "'}'", Some(&format!("'{}'", self.current_token().symbol()))))
		}

		self.advance(None).unwrap_or(());
		Ok(())
	}

	fn if_stmt(&mut self) -> Result<Node, Feedback> {
		let mut node = Node::IfStmt { condition: Box::new(Node::Undefined), body: Box::new(Node::block(vec![]))};
		self.control_flow(&mut node)?;
		Ok(node)
	}

	fn loop_stmt(&mut self) -> Result<Node, Feedback> {
		let mut node = Node::LoopStmt { condition: Box::new(Node::Undefined), body: Box::new(Node::block(vec![])) };
		self.control_flow(&mut node)?;
		Ok(node)
	}

	fn type_node(&mut self) -> Result<Node, Feedback> {
		match self.current_token().token_type() {
			TokenType::Identifier => Ok(Node::Type { identifier: Box::new(Node::identifier(self.current_token().symbol(), self.current_token().pos())) }),
			TokenType::Operator if self.current_token().symbol() == "*" => self.type_ptr_node(),
			_ => Err(Error::expected(self.current_token().pos(), "type", Some(&format!("'{}'", self.current_token().symbol()))))
		}
	}

	fn type_ptr_node(&mut self) -> Result<Node, Feedback> {
		if let Err(Some(feedback)) = self.advance(Some("'mut' or type")) {
			return Err(feedback);
		}

		let mutable = match self.current_token().token_type() {
			TokenType::Keyword if self.current_token().symbol() == "mut" => {
				if let Err(Some(feedback)) = self.advance(Some("type")) {
					return Err(feedback);
				}

				true
			}
			_ => false
		};

		Ok(Node::TypePtr { hodling_type: Box::new(self.type_node()?), mutable })
	}
}

pub fn parse(input: &str) -> Result<Node, Feedback> {
	let lexer_gen = lex::LexerGen::new()

	todo!();
}
*/

use parse::*;

#[derive(Debug, Clone)]
pub enum Item {
	Func(Func),
	FuncProto(FuncProto)
}

#[derive(Debug, Clone)]
pub struct Label {
	pub id: String,
	pub item: Item
}

#[derive(Debug, Clone)]
pub struct Func {
	pub stmts: Stmts
}

#[derive(Debug, Clone)]
pub struct FuncProto;

#[derive(Debug, Clone)]
pub enum Stmt {
	Label(Label)
}

#[derive(Debug, Clone)]
pub struct Stmts {
	pub stmts: Vec<Stmt>
}

#[derive(Debug, Clone)]
pub struct Expr {
	pub value: f64
}

#[derive(Debug, Clone)]
pub enum Node {
	Token(Token),
	// ----------
	NewLine,
	OptNewLine(bool),
	Item(Item),
	Label(Label),
	Func(Func),
	FuncProto(FuncProto),
	Stmt(Stmt),
	Stmts(Stmts),
	Expr(Expr),
	Program(Option<Stmts>)
}

impl ASTNode for Node {
	fn new_token(token: &Token) -> Self {
		Self::Token(token.to_owned())
	}

	fn token(&self) -> Result<&Token, String> {
		match self {
			Self::Token(token) => Ok(token),
			_ => Err("Node is not a token".to_owned())
		}
	}

	fn is_token(&self) -> bool {
		matches!(self, Self::Token(_))
	}
}

fn expr_num(nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::Expr(Expr { value: nodes[0].token().unwrap().symbol().parse::<f64>().unwrap() }))
}

fn expr_op(nodes: &[Node]) -> Result<Node, String> {
	let left = match &nodes[0] {
		Node::Token(x) if x.name() == "NUM" => x,
		_ => return Err(format!("Invalid node '{:?}' in 'expr_op'", nodes[0]))
	};

	let op = match &nodes[1] {
		Node::Token(x) => x,
		_ => return Err(format!("Invalid node '{:?}' in 'expr_op'", nodes[1]))
	};

	let right = match &nodes[2] {
		Node::Expr(x) => x,
		_ => return Err(format!("Invalid node '{:?}' in 'expr_op'", nodes[2]))
	};

	let value = match op.name().as_str() {
		"MINUS" => left.symbol().parse::<f64>().unwrap() - right.value,
		"PLUS" => left.symbol().parse::<f64>().unwrap() + right.value,
		"MULT" => left.symbol().parse::<f64>().unwrap() * right.value,
		"DIV" => left.symbol().parse::<f64>().unwrap() / right.value,
		_ => return Err(format!("Invalid operator '{}' in expr_op", op.name()))
	};

	Ok(Node::Expr(Expr { value }))
}

fn func(nodes: &[Node]) -> Result<Node, String> {
	let stmts = match &nodes[4] {
		Node::Stmts(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'func'", nodes[4]))
	};

	Ok(Node::Func(Func { stmts }))
}

fn func_proto(_nodes: &[Node]) -> Result<Node, String> {
	Ok(Node::FuncProto(FuncProto))
}

fn item(nodes: &[Node]) -> Result<Node, String> {
	match &nodes[0] {
		Node::Func(x) => Ok(Node::Item(Item::Func(x.to_owned()))),
		Node::FuncProto(x) => Ok(Node::Item(Item::FuncProto(x.to_owned()))),
		_ => Err(format!("Invalid node '{:?}' in 'item'", nodes[0]))
	}
}

fn label(nodes: &[Node]) -> Result<Node, String> {
	let id = match &nodes[0] {
		Node::Token(x) if x.name() == "ID" => x.symbol().to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'label'", nodes[0]))
	};

	let item = match &nodes[2] {
		Node::Item(x) => x.to_owned(),
		_ => return Err(format!("Invalid node '{:?}' in 'label'", nodes[2]))
	};

	Ok(Node::Label(Label { id, item }))
}

fn opt_new_line(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::OptNewLine(false))
	}

	match &nodes[0] {
		Node::Token(x) if x.name() == "NL" => Ok(Node::OptNewLine(true)),
		_ => Err(format!("Invalid node '{:?}' in 'opt_new_line'", nodes[0]))
	}
}

fn program(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Program(None));
	}

	match &nodes[0] {
		Node::Stmts(x) => Ok(Node::Program(Some(x.to_owned()))),
		_ => Err(format!("Invalid node '{:?}' in 'program'", nodes[0]))
	}
}

fn stmt(nodes: &[Node]) -> Result<Node, String> {
	match &nodes[0] {
		Node::Label(x) => Ok(Node::Stmt(Stmt::Label(x.to_owned()))),
		_ => Err(format!("Invalid node '{:?}' in 'stmt'", nodes[0]))
	}
}

fn stmts(nodes: &[Node]) -> Result<Node, String> {
	if nodes.is_empty() {
		return Ok(Node::Stmts(Stmts { stmts: vec![] }));
	}

	match &nodes[0] {
		Node::Stmt(x) => Ok(Node::Stmts(Stmts { stmts: vec![x.to_owned()] })),
		_ => Err(format!("Invalid node '{:?}' in 'stmts'", nodes[0]))
	}
}

pub fn parse(input: &str) -> Result<Node, (ParserError, Position)> {
	let mut lexer_builder = LexerBuilder::new();

	lexer_builder.ignore_rule(r"(^[ \t]+)").unwrap();
	lexer_builder.add_rules(&[
		("COL",   r"(^[:])"),
		("DIV",   r"(^[/])"),
		("FUNC",  r"(^func)"),
		("ID",    r"(^[a-zA-Z_][a-zA-Z0-9_]*)"),
		("LCBR",  r"(^[{])"),
		("LPAR",  r"(^[(])"),
		("MINUS", r"(^[-])"),
		("MULT",  r"(^[*])"),
		("NL",    r"(^[\r\n]+)"),
		("NUM",   r"(^\d+(\.\d+)?)"),
		("PLUS",  r"(^[+])"),
		("RCBR",  r"(^[}])"),
		("RPAR",  r"(^[)])")
	]).unwrap();

	let lexer = lexer_builder.build();
/*
	for token in lexer.lex(&input) {
		match token {
			Ok(token) => println!("{:#?}", token),
			Err(e) => {
				println!("{:?}", e);
				break;
			}
		}
	}
*/
	let mut parser_builder = parse::ParserBuilder::<Node>::new(&lexer.rules().iter().map(|x| x.name().as_str()).collect::<Vec<&str>>());

	parser_builder.add_patterns(&[
		("expr",       "NUM PLUS expr", expr_op),
		("expr",       "NUM MINUS expr", expr_op),
		("expr",       "NUM MULT expr", expr_op),
		("expr",       "NUM DIV expr", expr_op),
		("expr",       "NUM", expr_num),
		("func",       "func_proto opt_nl LCBR opt_nl stmts opt_nl RCBR", func),
		("func_proto", "FUNC", func_proto),
		("item",       "func", item),
		("item",       "func_proto", item),
		("label",      "ID COL item", label),
		("opt_nl",     "NL", opt_new_line),
		("opt_nl",     "", opt_new_line),
		("program",    "stmts", program),
		("program",    "", program),
		("stmt",       "label", stmt),
		("stmts",      "stmt", stmts),
		("stmts",      "", stmts),
	]).unwrap();

	let mut parser = parser_builder.build();
	parser.parse(lexer.lex(&input))
}