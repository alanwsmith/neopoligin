use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::sequence::pair;
use nom::IResult;
use nom::sequence::preceded;
use nom::character::complete::not_line_ending;
use nom::Parser;

pub fn vimeo_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- vimeo")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, id) = opt(preceded(tag("-- "), not_line_ending.map(|n: &str| n.to_string())))(source)?;
      Ok((
        source,
        NeoSection::Vimeo {
            attributes: None,
            id
        },
    ))
}
