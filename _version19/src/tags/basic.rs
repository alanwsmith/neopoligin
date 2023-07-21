use crate::tags::Tag;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::sequence::tuple;
use nom::sequence::delimited;
use nom::IResult;
use crate::tag_attrs::tag_attrs;
use nom::Parser;

pub fn basic<'a>(source: &'a str, name: &'a str) -> IResult<&'a str, Tag> {
    let (source, text) =
        delimited(tag("<<"), is_not("|").map(|s: &str| s.to_string()), tuple((tag("|"), tag_no_case(name))))(source)?;
    let (source, attrs) = tag_attrs(source)?;
    let (source, _) = tag(">>")(source)?;
    match name {
        "abbr" => {
            Ok((source, Tag::Abbr { text, attrs} ))
        },
        "b" => {
            Ok((source, Tag::B{ text, attrs} ))
        },
        "dfn" => {
            Ok((source, Tag::Dfn{ text, attrs} ))
        },
        "em" => {
            Ok((source, Tag::Em{ text, attrs} ))
        },
        "i" => {
            Ok((source, Tag::I{ text, attrs} ))
        },
        "kbd" => {
            Ok((source, Tag::Kbd{ text, attrs} ))
        },
        "mark" => {
            Ok((source, Tag::Mark{ text, attrs} ))
        },
        "q" => {
            Ok((source, Tag::Q { text, attrs} ))
        },
        "s" => {
            Ok((source, Tag::S { text, attrs} ))
        },
        "samp" => {
            Ok((source, Tag::Samp { text, attrs} ))
        },
        "small" => {
            Ok((source, Tag::Small{ text, attrs} ))
        },
        "span" => {
            Ok((source, Tag::Span { text, attrs} ))
        },
        "strong" => {
            Ok((source, Tag::Strong { text, attrs} ))
        },
        "sub" => {
            Ok((source, Tag::Sub { text, attrs} ))
        },
        "sup" => {
            Ok((source, Tag::Sup { text, attrs} ))
        },
        "u" => {
            Ok((source, Tag::U{ text, attrs} ))
        },
        "var" => {
            Ok((source, Tag::Var{ text, attrs} ))
        },
        "wbr" => {
            Ok((source, Tag::Wbr{ text, attrs} ))
        },
        _ => panic!("No tag")
    } 
}

#[cfg(test)]

mod test{
    use super::*;
    use rstest::rstest;
    use nom::error::Error;
    use nom::Err;
    use crate::tags::TagAttr;

    #[rstest]
    #[case(
        "<<delta|abbr>>", 
        "abbr",
        Ok(("", Tag::Abbr{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<delta|b>>", 
        "b",
        Ok(("", Tag::B{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<delta|dfn>>", 
        "dfn",
        Ok(("", Tag::Dfn{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<delta|i>>", 
        "i",
        Ok(("", Tag::I{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<delta|mark>>", 
        "mark",
        Ok(("", Tag::Mark{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<delta|q>>", 
        "q",
        Ok(("", Tag::Q{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<delta|s>>", 
        "s",
        Ok(("", Tag::S{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<delta|samp>>", 
        "samp",
        Ok(("", Tag::Samp{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<delta|small>>", 
        "small",
        Ok(("", Tag::Small{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<delta|span>>", 
        "span",
        Ok(("", Tag::Span{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<delta|sub>>", 
        "sub",
        Ok(("", Tag::Sub{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<delta|sup>>", 
        "sup",
        Ok(("", Tag::Sup{ attrs: vec![], text: "delta".to_string() })))]
    #[case(
        "<<alfa bravo|strong>>", 
        "strong",
        Ok(("", Tag::Strong{ attrs: vec![], text: "alfa bravo".to_string() })))]
    #[case(
        "<<alfa bravo|u>>", 
        "u",
        Ok(("", Tag::U{ attrs: vec![], text: "alfa bravo".to_string() })))]
    #[case(
        "<<alfa bravo|var>>", 
        "var",
        Ok(("", Tag::Var{ attrs: vec![], text: "alfa bravo".to_string() })))]
    #[case(
        "<<alfa bravo|wbr>>", 
        "wbr",
        Ok(("", Tag::Wbr{ attrs: vec![], text: "alfa bravo".to_string() })))]
    #[case(
        "<<alfa bravo|strong|class: charlie delta|id: echo>>", 
        "strong",
        Ok(("", 
        Tag::Strong{ 
                attrs: vec![
                    TagAttr::Class(
                        vec![
                            "charlie".to_string(), "delta".to_string()
                        ]
                    ), 
                    TagAttr::Id("echo".to_string())
                ],
                text: "alfa bravo".to_string() 
            })))]
    fn strong_runner(#[case] i1: &str, #[case] i2: &str, #[case] e: Result<(&str, Tag), Err<Error<&str>>>) {
        assert_eq!(e, basic(i1, i2))
    }
}
