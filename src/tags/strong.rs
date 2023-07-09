use crate::tags::basic::basic;
use nom::IResult;
use crate::tags::Tag;

pub fn strong(source: &str) -> IResult<&str, Tag> {
    basic(source, "strong")
}
