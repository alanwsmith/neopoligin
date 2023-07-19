use crate::attrs::Attribute;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::IResult;
use nom::character::complete::space1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::bytes::complete::is_not;
use nom::character::complete::space0;

pub fn generic(source: &str) -> IResult<&str, Attribute> {
    let (source, attr) = 
    pair(
        preceded(tag("-- "), is_not(":")),
        preceded(
            pair(tag(":"), space1),
            not_line_ending
        )
    )(source)?;
    let (source, _) = pair(space0, line_ending)(source)?;
    Ok((source, Attribute::Generic(attr.0.to_string(), attr.1.to_string())))
}
