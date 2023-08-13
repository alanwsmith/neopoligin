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
use crate::blocks::block;

pub fn hr_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- hr")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = opt(many1(attribute))(source)?;
    let (source, body) = opt(many1(block))(source)?;
     Ok((
        source,
        NeoSection::Hr {
            attributes,
            body,
        },
    ))
}
