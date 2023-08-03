use crate::neo_sections::NeoSection;
use crate::neo_sections::attribute;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::error::VerboseError;
use nom::sequence::pair;
use nom::IResult;
use nom::multi::many1;
use nom::combinator::opt;

pub fn starttldr_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- starttldr")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = opt(many1(attribute))(source)?;
     Ok((
        source,
        NeoSection::StartTlDr {
            attributes,
        },
    ))
}
