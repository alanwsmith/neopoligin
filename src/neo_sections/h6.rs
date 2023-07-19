use crate::attrs::attrs;
use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
use nom::sequence::tuple;
use nom::IResult;
use nom::bytes::complete::take_until;
use nom::combinator::opt;
use nom::Parser;
use nom::combinator::rest;
use nom::branch::alt;

pub fn h6(source: &str) -> IResult<&str, NeoSection> {
    let (source, _) = tuple((
        multispace0,
        tag_no_case("-- h6"),
        not_line_ending,
        newline,
    ))(source)?;
    let (source, attrs) = attrs(source)?;
    let (source, text) = opt(alt((take_until("\n-- "), rest)).map(|d: &str| d.trim().to_string()))(source)?;
    Ok((
        source,
        NeoSection::H6 {
            attrs,
            text,
        },
    ))
}
