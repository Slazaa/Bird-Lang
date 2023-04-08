use nom::{
    IResult, Parser,
    combinator::opt
};

use nom_supreme::{
    error::ErrorTree,
    tag::complete::tag
};

#[derive(Debug)]
pub enum Vis {
	Private,
	Public
}

impl Vis {
    pub fn parse(input: &str) -> IResult<&str, Self, ErrorTree<&str>> {
        opt(tag("pub")).map(|e| if e.is_some() { Vis::Public } else { Vis::Private }).parse(input)
    }
}
