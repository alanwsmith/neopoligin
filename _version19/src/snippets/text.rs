use crate::snippets::Snippet;
use nom::bytes::complete::is_not;
use nom::IResult;

pub fn text(source: &str) -> IResult<&str, Snippet> {
    let (source, text_string) = is_not("<")(source)?;
    Ok((
        source,
        Snippet::Text {
            text: text_string.to_string(),
        },
    ))
}
