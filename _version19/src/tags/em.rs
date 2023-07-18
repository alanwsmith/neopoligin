use crate::tags::basic::basic;
use crate::tags::Tag;
use nom::IResult;

pub fn em(source: &str) -> IResult<&str, Tag> {
    basic(source, "em")
}
