// use crate::blocks::list_paragraph_block::list_paragraph_block;
use crate::blocks::paragraph_block::paragraph_block;
use crate::containers::Container;
use nom::bytes::complete::tag;
// use nom::character::complete::multispace0;
use nom::error::VerboseError;
use nom::multi::many1;
// use nom::sequence::preceded;
use nom::IResult;
// use nom::sequence::terminated;
// use nom::combinator::opt;
use nom::character::complete::multispace0;

pub fn list_item_container(source: &str) -> IResult<&str, Container, VerboseError<&str>> {
    
    // dbg!(source);
    let (source, _) = multispace0(source)?;
    let (source, _) = tag("- ")(source)?;
    // dbg!(source);
    let (source, body) = many1(paragraph_block)(source)?;
    // dbg!(source);
    // dbg!(source);
    Ok((source, Container::ListItem { body }))
}
