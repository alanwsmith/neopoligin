use crate::attributes::attribute;
use crate::containers::list_item_container::*;
use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::pair;
use nom::IResult;

pub fn warnings_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- warnings")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = opt(many1(attribute))(source)?;
    let (source, items) = opt(many1(list_item_container))(source)?;
    Ok((
        source,
        NeoSection::Warnings {
            attributes,
            items,
            preface: None,
        },
    ))
}
