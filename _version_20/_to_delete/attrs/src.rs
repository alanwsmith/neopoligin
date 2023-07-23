use crate::attrs::Attribute;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::sequence::delimited;
use nom::IResult;

pub fn src(source: &str) -> IResult<&str, Attribute> {
    let (source, attr) = delimited(tag("-- src: "), not_line_ending, line_ending)(source)?;
    Ok((source, Attribute::Src(attr.to_string())))
}
