use crate::tags::basic::basic;
use nom::IResult;
use crate::tags::Tag;

pub fn b(source: &str) -> IResult<&str, Tag> {
    basic(source, "b")
}
