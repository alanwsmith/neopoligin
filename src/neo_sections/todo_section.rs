use crate::attributes::attribute;
use crate::containers::todo_item_container::*;
use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::pair;
use nom::IResult;

pub fn todo_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- todo")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = opt(many1(attribute))(source)?;
    let (source, items) = opt(many1(todo_item_container))(source)?;
    Ok((
        source,
        NeoSection::Todo {
            attributes,
            items,
            preface: None,
        },
    ))
}
