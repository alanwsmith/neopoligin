use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
use nom::sequence::tuple;
use nom::IResult;
use nom::character::complete::line_ending;
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::multi::many1;
use nom::Parser;


pub fn categories(source: &str) -> IResult<&str, NeoSection> {
    let (source, _) = tuple((
        multispace0,
        tag_no_case("-- categories"),
        not_line_ending,
        newline,
    ))(source)?;
    let(source, list) = many1(
        delimited(
            tag("-- "), 
            not_line_ending, 
            line_ending
        ).map(|c: &str| c.to_string())
    )(source)?;
    Ok((
        source,
        NeoSection::Categories{list}
    ))
}
