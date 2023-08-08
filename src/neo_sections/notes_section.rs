use crate::attributes::attribute;
use crate::blocks::block;
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

pub fn notes_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    // dbg!(&source);
    let (source, _) = tag("-- notes")(source)?;
    // dbg!(&source);
    let (source, _) = pair(space0, line_ending)(source)?;
    // dbg!(&source);
    let (source, attributes) = opt(many1(attribute))(source)?;
    // dbg!(&source);
    let (source, prelude) = opt(many1(block))(source)?;
    // dbg!(&prelude);
    // dbg!(&source);
    let (source, items) = opt(many1(list_item_container))(source)?;
    // dbg!(&source);
    Ok((
        source,
        NeoSection::Notes {
            attributes,
            items,
            prelude,
        },
    ))
}
