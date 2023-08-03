#![allow(unused_imports)]
use crate::attributes::*;
use crate::blocks::block;
use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::IResult;

pub fn h2_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- h2")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = opt(many1(attribute))(source)?;
    let (source, headline) = opt(block)(source)?;
    let (source, body) = opt(many1(block))(source)?;
    Ok((
        source,
        NeoSection::H2 {
            attributes,
            body,
            headline,
        },
    ))
}
