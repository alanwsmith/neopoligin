use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::error::VerboseError;
use nom::sequence::pair;
use nom::IResult;

pub fn startdiv_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- startdiv")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
     Ok((
        source,
        NeoSection::StartDiv {
            attributes: None,
        },
    ))
}
