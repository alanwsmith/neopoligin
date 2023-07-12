use crate::tag_attrs::TagAttr;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::IResult;

pub fn caption(source: &str) -> IResult<&str, TagAttr> {
    let (source, value_string) = preceded(tag("|caption: "), is_not("|>"))(source)?;
    Ok((source, TagAttr::Caption(value_string.to_string())))
}
