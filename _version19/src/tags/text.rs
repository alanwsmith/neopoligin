use crate::tags::Tag;
use nom::bytes::complete::is_not;
use nom::IResult;

pub fn text(source: &str) -> IResult<&str, Tag> {
    let (source, text_string) = is_not("<")(source)?;
    Ok((
        source,
        Tag::Text {
            text: text_string.to_string(),
        },
    ))
}
