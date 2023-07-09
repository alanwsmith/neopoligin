#![allow(unused_imports)]
use crate::tag_attrs::tag_attrs;
use crate::tags::Tag;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::sequence::terminated;
use nom::branch::alt;
use nom::IResult;
use nom::Parser;
use nom::combinator::peek;
use crate::tag_attrs::id::id;
use nom::multi::many0;
use crate::tags::TagAttr;
use nom::character::complete::space1;
use nom::multi::separated_list1;


pub fn code(source: &str) -> IResult<&str, Tag> {
    let (source, text) = delimited(
        tag("<<"),
        is_not("|").map(|s: &str| s.to_string()),
        tuple((tag("|"), tag_no_case("code"))),
    )(source)?;
    let (source, lang) = opt(
        preceded(
            tag("|"),
            terminated(
                is_not("|:>").map(|s: &str| s.to_string()),
                alt((
                    peek(tag("|")), peek(tag(">"))
                ))
            )
        )
    )(source)?;

    let (source, mut attrs) = tag_attrs(source)?;
    let (source, _) = tag(">>")(source)?;

    let found_it = attrs.iter_mut().find(|x| match x {
        TagAttr::Class(_) => {true},
        _ => { false }
    });

    match (found_it, lang) {
        (Some(TagAttr::Class(the_thing)), Some(the_lang)) => {
            the_thing.push(format!("language-{}", the_lang));
        }
        (None, Some(the_lang)) => {
            attrs.push(TagAttr::Class(vec![format!("language-{}", the_lang)]))
        },
        _ => {}
    }
     Ok((source, Tag::Code { attrs, text}))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tag_attrs::TagAttr;
    use rstest::rstest;

    #[rstest]
    #[case(
        "<<foxtrot|code>>",
        Tag::Code{ attrs: vec![], text: "foxtrot".to_string() }
    )]
    #[case(
        "<<sierra|code|rust>>",
        Tag::Code{ 
            attrs: vec![
                TagAttr::Class(vec!["language-rust".to_string()])
            ], 
            text: "sierra".to_string() }
    )]
    #[case(
        "<<foxtrot|code|python|id: bravo|class: echo>>",
        Tag::Code{ attrs: vec![
            TagAttr::Id("bravo".to_string()),
            TagAttr::Class(vec!["echo".to_string(), "language-python".to_string()])
        ],  text: "foxtrot".to_string() }
    )]
    #[case(
        "<<bravo|code|js|id: delta>>",
        Tag::Code{ attrs: vec![
            TagAttr::Id("delta".to_string()),
            TagAttr::Class(vec!["language-js".to_string()])
        ],  text: "bravo".to_string() }
    )]
    #[case(
        "<<alfa|code|class: echo>>",
        Tag::Code{ 
            attrs: vec![
                TagAttr::Class(vec!["echo".to_string()])
            ], 
            text: "alfa".to_string() }
    )]
    fn link_test(#[case] i: &str, #[case] e: Tag) {
        assert_eq!(e, code(i).unwrap().1);
    }

}
