use crate::tags::basic::basic;
use nom::IResult;
use crate::tags::Tag;

pub fn sub(source: &str) -> IResult<&str, Tag> {
    basic(source, "sub")
}
