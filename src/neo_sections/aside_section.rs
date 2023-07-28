use crate::attributes::*;
use crate::blocks::block;
use crate::helpers::empty_line::empty_line;
use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::pair;
use nom::IResult;

pub fn aside_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- aside")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, _attributes) = attributes(source)?;
    let (source, _) = empty_line(source)?;
    let (source, content) = opt(many1(block))(source)?;
    Ok((
        source,
        NeoSection::Aside {
            attributes: vec![AttributeV2::Id("tango".to_string())],
            content,
        },
    ))
}
