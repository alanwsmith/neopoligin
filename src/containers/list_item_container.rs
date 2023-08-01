use crate::blocks::paragraph_block::paragraph_block;
use crate::containers::Container;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::multi::many1;
use nom::IResult;
use nom::character::complete::multispace0;

pub fn list_item_container(source: &str) -> IResult<&str, Container, VerboseError<&str>> {
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("- ")(source)?;
    let (source, body) = many1(paragraph_block)(source)?;
    Ok((source, Container::ListItem { body }))
}
