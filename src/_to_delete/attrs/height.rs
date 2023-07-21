use crate::attrs::Attribute;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::sequence::delimited;
use nom::IResult;

pub fn height(source: &str) -> IResult<&str, Attribute> {
    let (source, attr) = delimited(tag("-- height: "), not_line_ending, line_ending)(source)?;
    Ok((source, Attribute::Height(attr.parse::<u32>().unwrap())))
}
