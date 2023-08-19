use crate::attributes::attribute;
use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::pair;
use nom::IResult;
use nom::Parser;

pub fn htmlstart_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- htmlstart")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = opt(many1(attribute))(source)?;
    let (source, body) = opt(take_until("-- htmlend").map(|d: &str| d.trim().to_string()))(source)?;
    Ok((source, NeoSection::Html { attributes, body }))
}
