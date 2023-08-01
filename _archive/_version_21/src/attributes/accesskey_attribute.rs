use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::sequence::preceded;
use nom::IResult;

use crate::attributes::Attribute;

pub fn accesskey_attribute(source: &str) -> IResult<&str, Attribute, VerboseError<&str>> {
    let (source, _) = space0(source)?;
    let (source, attr) = preceded(tag("accesskey: "), is_not("|>\n"))(source)?;
    let (source, _) = opt(line_ending)(source)?;
    Ok((source, Attribute::AccessKey(attr.to_string())))
}
