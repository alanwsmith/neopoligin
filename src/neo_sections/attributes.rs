use crate::attrs::attrs;
use crate::blocks::paragraph::paragraph;
use crate::neo_sections::NeoSection;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;

pub fn attributes(source: &str) -> IResult<&str, NeoSection> {
    let (source, _) = tuple((
        multispace0,
        tag_no_case("-- attributes"),
        not_line_ending,
        newline,
    ))(source)?;
    let (source, attrs) = attrs(source)?;
   Ok((
        source,
        NeoSection::Attributes {
            attrs,
        },
    ))
}
