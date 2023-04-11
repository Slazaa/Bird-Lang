use nom::{IResult, Parser};
use nom_supreme::{error::ErrorTree, ParserExt, tag::complete::tag};

use super::{Expr, ws};

#[derive(Debug, Clone)]
pub struct Return<'a> {
    pub value: Expr<'a>
}

impl<'a> Return<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
        ws(Expr::parse).preceded_by(tag("return"))
            .parse(input)
            .map(|(input, value)| {
                (input, Self { value })
            })
    }
}