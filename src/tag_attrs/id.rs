use crate::tag_attrs::TagAttr;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::IResult;

pub fn id(source: &str) -> IResult<&str, TagAttr> {
    let (source, value_string) = preceded(tag("|id: "), is_not("|>"))(source)?;
    Ok((source, TagAttr::Id(value_string.to_string())))
}
