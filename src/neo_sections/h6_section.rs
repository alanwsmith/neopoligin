use crate::attributes::attributes;



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

pub fn h6_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    let (source, _) = tag("-- h6")(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    let (source, attributes) = attributes(source)?;
    let (source, _) = empty_line(source)?;
    let (source, headline) = opt(block)(source)?;
    let (source, body) = opt(many1(block))(source)?;
    Ok((
        source,
        NeoSection::H6 {
            attributes: Some(attributes),
            body,
            headline,
        },
    ))
}
