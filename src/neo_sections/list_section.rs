// use crate::attributes::*;
use crate::containers::list_item_container::*;
// use crate::blocks::block;
// use crate::helpers::empty_line::empty_line;
use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::sequence::pair;
use nom::IResult;

pub fn list_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    // dbg!(source);
    let (source, _) = tag("-- list")(source)?;
    // dbg!(source);
    let (source, _) = pair(space0, line_ending)(source)?;
    // dbg!(source);

    // let (source, attributes) = attributes_v2(source)?;
    // let (source, _) = empty_line(source)?;
    let (source, items) = opt(many1(list_item_container))(source)?;
    // dbg!(source);
    Ok((
        source,
        NeoSection::List {
            attributes: None,
            items,
            preface: None,
        },
    ))
}
