use crate::attributes::*;
use crate::helpers::empty_line::empty_line;
use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::error::VerboseError;
use nom::sequence::pair;
use nom::IResult;


pub fn startcode_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- startcode")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = attributes_v2(source)?;
    let (source, _) = empty_line(source)?;
    let (source, body) = take_until("\n\n-- endcode")(source)?;
    Ok((
        source,
        NeoSection::Code {
            attributes,
            body: Some(body.trim().to_string())
        },
    ))
}
