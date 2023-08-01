use crate::tags::basic::basic;
use nom::IResult;
use crate::tags::Tag;

pub fn u(source: &str) -> IResult<&str, Tag> {
    basic(source, "u")
}
