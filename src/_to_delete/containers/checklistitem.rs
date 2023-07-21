use crate::containers::Container;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::rest;
use nom::IResult;
use crate::blocks::paragraph::paragraph;
use nom::combinator::eof;
use nom::multi::many_till;

pub fn checklistitem(source: &str) -> IResult<&str, Container> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("[] ")(source)?;
    let (source, captured) = alt((take_until("\n[]"), rest))(source)?;
    let (_, blocks) = many_till(paragraph, eof)(captured)?;
    Ok((
        source,
        Container::ChecklistItem {
            blocks: Some(blocks.0)
        },
    ))
}
