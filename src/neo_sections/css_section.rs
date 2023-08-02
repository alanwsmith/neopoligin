use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::sequence::pair;
use nom::IResult;
use nom::bytes::complete::take_until;
use nom::Parser;

pub fn css_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- css")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, body) = opt(take_until("-- ").map(|d: &str| d.trim().to_string()))(source)?;
    Ok((
        source,
        NeoSection::Css {
            attributes: None,
            body,
        },
    ))
}


