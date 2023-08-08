use crate::neo_sections::NeoSection;
use crate::attributes::attribute;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::sequence::pair;
use nom::IResult;
use nom::multi::many1;
use nom::bytes::complete::take_until;
use nom::Parser;

pub fn startcss_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- startcss")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    dbg!(source);
    let (source, attributes) = opt(many1(attribute))(source)?;
    dbg!(source);
    let (source, body) = opt(take_until("-- endcss").map(|d: &str| d.trim().to_string()))(source)?;
    Ok((
        source,
        NeoSection::Css {
            attributes,
            body,
        },
    ))
}


