use crate::bird::constants::*;
use crate::bird::feedback::*;
use crate::bird::lexer::*;
use crate::bird::parser::parse::*;

/// This type represents the functions that create a `Node`
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
	match parser.current_token().token_type() {
		TokenType::Literal => {
			let literal = Node::literal(parser.current_token().symbol(), parser.current_token().pos());
			parser.advance().unwrap_or(());

			return Ok(literal);
		}
		TokenType::Identifier => {
			let identifier = Node::identifier(parser.current_token().symbol(), parser.current_token().pos());
			parser.advance().unwrap_or(());
			
			return Ok(identifier);
		}
		TokenType::Operator if "+-".contains(parser.current_token().symbol()) => {
			let factor = factor(parser)?;
			let operator = Node::operator(parser.current_token().symbol(), parser.current_token().pos());

			parser.advance().unwrap_or(());

			return Ok(Node::UnaryExpr { operator: Box::new(operator), node: Box::new(factor) });
		}
		TokenType::Separator if "(".contains(parser.current_token().symbol()) => {
			if parser.advance().is_err() {
				return Err(Error::expected(parser.current_token().pos(), "expression", None))
			}

			let expr = expr(parser)?;
			
			if parser.advance().is_err() {
				return Err(Error::expected(parser.current_token().pos(), "')'", None))
			}

			if *parser.current_token().token_type() == TokenType::Separator && ")".contains(parser.current_token().symbol()) {
				parser.advance().unwrap_or(());
				return Ok(expr);
			}
			
			return Err(Error::expected(parser.current_token().pos(), "')'", Some(&format!("'{}'", parser.current_token().symbol()))));
		}
		_ => ()
	}

	if let Some(last_token) = parser.last_token() {
		let pos = last_token.pos();
		let mut pos = (pos.0.clone(), pos.1.clone());

		*pos.0.colomn_mut() += 2;
		*pos.1.colomn_mut() += 2;

		return Err(Error::expected((&pos.0, &pos.1), "number", Some(&format!("'{}'", parser.current_token().symbol()))));
	}

	Err(Error::expected(parser.current_token().pos(), "number", Some(&format!("'{}'", parser.current_token().symbol()))))
}

pub fn binary_op(parser: &mut Parser, first_func: NodeFunc, second_func: Option<NodeFunc>, ops: Vec<&str>) -> Result<Node, Feedback> {
	let mut func = first_func;

	if let Some(second_func) = second_func {
		func = second_func;
	}

	let second_func = func;
	let mut left = first_func(parser)?;

	if !parser.is_more_token() {
		return Ok(left);
	}

	if *parser.current_token().token_type() != TokenType::Operator && *parser.current_token().token_type() != TokenType::Separator {
		return Err(Error::expected(parser.current_token().pos(), "operator", Some(&format!("'{}'", parser.current_token().symbol()))));
	}
	
	while parser.is_more_token() {
		if !ops.contains(&parser.current_token().symbol()) {
			break;
		}

		if !OPERATORS.contains(&parser.current_token().symbol()) {
			return Err(Error::invalid_syntax(Some(parser.current_token().pos()), "Invalid operator"))
		}

		let operator = Node::operator(parser.current_token().symbol(), parser.current_token().pos());

		if parser.advance().is_err() {
			return Err(Error::expected(parser.current_token().pos(), "expression", None))
		}

		let old_left = left;
		let right = second_func(parser)?;

		left = Node::BinExpr { operator: Box::new(operator), left: Box::new(old_left), right: Box::new(right) };
	}

	Ok(left)
}