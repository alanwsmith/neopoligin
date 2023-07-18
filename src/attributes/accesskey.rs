use crate::attributes::Attribute;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::sequence::delimited;
use nom::IResult;

pub fn accesskey(source: &str) -> IResult<&str, Attribute> {
    let (source, attr) = delimited(tag("-- accesskey: "), not_line_ending, line_ending)(source)?;
    Ok((source, Attribute::AccessKey(attr.to_string())))
}
