use crate::tags::basic::basic;
use nom::IResult;
use crate::tags::Tag;

pub fn kbd(source: &str) -> IResult<&str, Tag> {
    basic(source, "kbd")
}
