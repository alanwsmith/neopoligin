use crate::attributes::*;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::pair;
use nom::IResult;
use crate::blocks::block;
use crate::helpers::empty_line::empty_line;
use crate::neo_sections::NeoSection;

pub fn endcode_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- endcode")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = attributes_v2(source)?;
    let (source, _) = empty_line(source)?;
    dbg!(source);
    let (source, body) = opt(many1(block))(source)?;
    dbg!(source);
    Ok((
        source,
        NeoSection::P {
            attributes,
            body,
        },
    ))
}
