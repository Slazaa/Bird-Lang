use nom::{IResult, sequence::tuple, Parser};
use nom_supreme::{error::ErrorTree, ParserExt, tag::complete::tag};

use super::{Expr, ws, path::Path};

#[derive(Debug, Clone)]
pub struct Assign<'a> {
    pub expr: Expr<'a>,
    pub value: Expr<'a>
}

impl<'a> Assign<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self, ErrorTree<&str>> {
        tuple((
            ws(Path::parse).map(|x| Expr::Path(x)).terminated(tag("=")),
            ws(Expr::parse)
        ))
            .parse(input)
            .map(|(input, (expr, value))| {
                (input, Self { expr, value })
            })
    }
}