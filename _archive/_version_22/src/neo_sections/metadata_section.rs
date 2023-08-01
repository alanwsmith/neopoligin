use crate::attributes::*;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::error::VerboseError;
use nom::sequence::pair;
use nom::IResult;
use crate::neo_sections::NeoSection;

pub fn metadata_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- metadata")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = attributes_v2(source)?;
    Ok((
        source,
        NeoSection::MetaData {
            attributes
        },
    ))
}
