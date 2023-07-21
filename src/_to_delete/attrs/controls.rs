use crate::attrs::Attribute;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::IResult;
use nom::sequence::tuple;
use nom::character::complete::space0;

pub fn controls(source: &str) -> IResult<&str, Attribute> {
    let (source, _) = tuple((tag("-- controls"), space0, line_ending))(source)?;
    Ok((source, Attribute::Controls))
}

