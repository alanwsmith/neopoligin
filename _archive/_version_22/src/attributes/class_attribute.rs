use crate::attributes::*;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;

pub fn class_attribute(source: &str) -> IResult<&str, AttributeV2, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, _attr) = tag("class:")(source)?;
    let (source, value) = many1(preceded(
        space0,
        is_not(" |>\n").map(|c: &str| c.to_string()),
    ))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, AttributeV2::Class(value)))
}

