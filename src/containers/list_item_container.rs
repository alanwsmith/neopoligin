

use crate::blocks::list_paragraph_block::list_paragraph_block;
use crate::containers::Container;


use nom::bytes::complete::tag;


use nom::character::complete::multispace0;








use nom::error::VerboseError;


use nom::multi::many1;


use nom::sequence::preceded;

use nom::IResult;





pub fn list_item_container(source: &str) -> IResult<&str, Container, VerboseError<&str>> {
    let (source, _) = tag("- ")(source)?;
    let (source, content) = many1(preceded(multispace0, list_paragraph_block))(source)?;
    Ok((source, Container::ListItem { content }))
}
