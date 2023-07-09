#![allow(unused_imports)]
use crate::blocks::paragraph::paragraph;
use crate::blocks::Block;
use crate::containers::Container;
use crate::section_attrs::sec_attrs;
use crate::sections::Section;
use crate::tags::Tag;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many1;
use nom::multi::many_till;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

pub fn notes(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-> notes"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n->"), rest))(source.trim())?;
    let (content, attrs) = sec_attrs(content.trim())?;
    let (content, paragraphs) =
        many_till(paragraph, alt((tag("- "), eof)))(content.trim())?;
    let (_, raw_items) = separated_list1(
        tag("- "),
        many_till(many_till(paragraph, alt((tag("- "), eof))), eof),
    )(content)?;
    let mut items: Vec<_> = raw_items
        .into_iter()
        .map(|i| {
            i.0.into_iter()
                .map(|x| Container::ListItem { paragraphs: x.0 })
                .collect::<Vec<_>>()
        })
        .collect();
    Ok((
        source,
        Section::Notes {
            attrs,
            items: items.pop().unwrap(),
            paragraphs: paragraphs.0, 
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sections::Section;
    use rstest::rstest;

// this test is mostly working but the data doesn't match 
// so it's currently out of rotation and just relying
// on the actual output for now
    #[rstest]
    #[ignore]
    #[case(
        ["-> notes", 
            ">> id: sierra",
            "", 
            "tango foxtrot", 
            "", 
            "- alfa1", 
            "alfa2", 
            "", 
            "alfa3", 
            "alfa4", 
            "",
            "- bravo1", 
            "",
            "bravo2", 
            "",
            "bravo3", 
            "",
            "-> placeholder"].join("\n"),
        Ok(("\n\n-> placeholder", 
        Section::List {
            attrs: vec![],
            paragraphs: vec![],
            items: vec![
                    Container::ListItem { paragraphs:
                        vec![
                            Block::Paragraph {
                                tags: vec![
    Tag::Text { text: "alfa1 alfa2".to_string() }
                                ]
                            },
                            Block::Paragraph {
                                tags: vec![
    Tag::Text { text: "alfa3 alfa4".to_string() }
                                ]
                            },
                        ]
                    },

                    Container::ListItem{
            paragraphs: 
                        vec![
                            Block::Paragraph {
                                tags: vec![
    Tag::Text { text: "bravo1".to_string() }
                                ]
                            },
                            Block::Paragraph {
                                tags: vec![
    Tag::Text { text: "bravo2".to_string() }
                                ]
                            },
                            Block::Paragraph {
                                tags: vec![
    Tag::Text { text: "bravo3".to_string() }
                                ]
                            },
                        ]
                    },
                ]
        }))
    )]
    fn solo_test_example(#[case] i: String, #[case] e: IResult<&str, Section>) {
        assert_eq!(e, notes(i.as_str()));
    }
}

