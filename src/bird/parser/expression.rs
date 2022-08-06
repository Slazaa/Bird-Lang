use crate::bird::constants::*;
use crate::bird::feedback::*;
use crate::bird::lexer::*;
use crate::bird::parser::parse::*;

pub type NodeFunc = fn(&mut Parser) -> Result<Node, Feedback>;

pub fn expr(parser: &mut Parser) -> Result<Node, Feedback> {
	binary_op(parser, comp_expr, None, vec!["&&", "||"])
}

pub fn comp_expr(parser: &mut Parser) -> Result<Node, Feedback> {
	binary_op(parser, arith_expr, None, vec!["==", "!=", ">", "<", ">=", "<="])
}

pub fn arith_expr(parser: &mut Parser) -> Result<Node, Feedback> {
	binary_op(parser, term, None, vec!["+", "-"])
}

pub fn term(parser: &mut Parser) -> Result<Node, Feedback> {
	binary_op(parser, factor, None, vec!["*", "/", "%"])
}

pub fn factor(parser: &mut Parser) -> Result<Node, Feedback> {
	let mut current_token = match parser.current_token() {
		Some(x) => x.clone(),
		None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
	};

	match current_token.token_type() {
		TokenType::Literal => {
			parser.advance();
			return Ok(Node::Literal(current_token.symbol().to_owned()));
		}
		TokenType::Identifier => {
			parser.advance();
			return Ok(Node::Identifier(current_token.symbol().to_owned()));
		}
		TokenType::Operator if "+-".contains(current_token.symbol()) => {
			parser.advance();
			let factor = factor(parser)?;
			return Ok(Node::UnaryExpr { operator: current_token.symbol().to_owned(), node: Box::new(factor) });
		}
		TokenType::Separator if "(".contains(current_token.symbol()) => {
			parser.advance();
			let expr = expr(parser)?;

			current_token = match parser.current_token() {
				Some(x) => x.clone(),
				None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
			};

			if *current_token.token_type() == TokenType::Separator && ")".contains(current_token.symbol()) {
				parser.advance();
				return Ok(expr);
			} else {
				current_token = match parser.current_token() {
					Some(x) => x.clone(),
					None => return Err(Error::invalid_syntax(None, "Invalid syntax"))
				};

				return Err(Error::invalid_syntax(Some(current_token.pos()), "Expected ')'"));
			}
		}
		_ => ()
	}

	if let Some(last_token) = parser.last_token() {
		let pos = last_token.pos();
		let mut pos = (pos.0.clone(), pos.1.clone());

		*pos.0.colomn_mut() += 2;
		*pos.1.colomn_mut() += 2;

		return Err(Error::expected((&pos.0, &pos.1), "Expected number", None));
	}

	Err(Error::expected(current_token.pos(), "number", Some(&format!("'{}'", current_token.symbol()))))
}

pub fn binary_op(parser: &mut Parser, first_func: NodeFunc, second_func: Option<NodeFunc>, ops: Vec<&str>) -> Result<Node, Feedback> {
	let mut func = first_func;

	if let Some(second_func) = second_func {
		func = second_func;
	}

	let second_func = func;
	let mut left = first_func(parser)?;

	if let Some(token) = parser.current_token() {
		if *token.token_type() != TokenType::Operator && *token.token_type() != TokenType::Separator {
			return Err(Error::expected(token.pos(), "operator", Some(&format!("'{}'", token.symbol()))));
		}
		
		while let Some(token) = parser.current_token() {
			if !ops.contains(&token.symbol()) {
				break;
			}

			if !OPERATORS.contains(&token.symbol()) {
				return Err(Error::invalid_syntax(Some(token.pos()), "Invalid operator"))
			}

			let operator = token.symbol().to_owned();

			parser.advance();

			let old_left = left;
			let right = second_func(parser)?;

			left = Node::BinExpr { operator, left: Box::new(old_left), right: Box::new(right) };
		}
	}

	Ok(left)
}