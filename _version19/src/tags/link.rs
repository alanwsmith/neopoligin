use crate::tag_attrs::tag_attrs;
use crate::tags::Tag;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

pub fn link(source: &str) -> IResult<&str, Tag> {
    let (source, text) = delimited(
        tag("<<"),
        is_not("|").map(|s: &str| s.to_string()),
        tuple((tag("|"), tag_no_case("link"))),
    )(source)?;
    let (source, url) = preceded(tag("|"), is_not("|>"))(source)?;
    let (source, attrs) = tag_attrs(source)?;
    let (source, _) = tag(">>")(source)?;

    Ok((
        source,
        Tag::Link {
            attrs,
            text: text.to_string(),
            url: url.to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tag_attrs::TagAttr;
    use rstest::rstest;

    #[rstest]
    #[case(
        "<<alfa|link|https://www.example.com/>>", 
        Tag::Link { attrs: vec![], text: "alfa".to_string(), url: "https://www.example.com/".to_string() }
    )]
    #[case(
        "<<alfa|link|https://www.example.com/|class: bravo>>", 
        Tag::Link { attrs: vec![
            TagAttr::Class(vec!["bravo".to_string()])
        ], text: "alfa".to_string(), url: "https://www.example.com/".to_string() }
    )]
    fn link_test(#[case] i: &str, #[case] e: Tag) {
        assert_eq!(e, link(i).unwrap().1);
    }
}
