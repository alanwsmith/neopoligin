use crate::tags::basic::basic;
use nom::IResult;
use crate::tags::Tag;

pub fn mark(source: &str) -> IResult<&str, Tag> {
    basic(source, "mark")
}
