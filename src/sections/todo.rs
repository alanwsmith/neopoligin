use nom::Parser;
use crate::blocks::paragraph::paragraph;
use nom::character::complete::multispace1;
use nom::character::complete::multispace0;
use nom::bytes::complete::is_not;
use nom::combinator::opt;
use nom::combinator::peek;
use nom::multi::many0;
use crate::containers::Container;
use crate::section_attrs::sec_attrs;
use crate::sections::Section;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum TodoStatus {
    Done,
    NotDone,
    Other(String)
}

pub fn todo(source: &str) -> IResult<&str, Section> {
    let (source, _) =
        tuple((tag_no_case("-> todo"), not_line_ending, line_ending))(
            source.trim(),
        )?;
    let (source, content) = alt((take_until("\n\n->"), rest))(source.trim())?;
    let (content, attrs) = sec_attrs(content.trim())?;
    let (content, paragraphs) =
         many_till(paragraph, alt((peek(tag("[")), eof)))(content.trim())?;

    let (_content, raw_items) = many0(
        tuple((
            multispace0,
            tag("["),
            opt(is_not("]")), 
            tag("]"),
            multispace1,
            alt((take_until("\n["), rest)).map(|x| 
             many_till(paragraph, eof)(x).unwrap().1.0
            )
        )))(content)?;


    let items: Vec<_> = raw_items.into_iter().map(|i| 
        match i.2 {
            None => {
        Container::TodoItem { status: TodoStatus::NotDone, paragraphs: i.5 }
            },
            Some(_x) => {
        Container::TodoItem { status: TodoStatus::Done, paragraphs: i.5 }
            }
        }
    ).collect();

    Ok((
        source,
        Section::Todo {
            attrs,
            paragraphs: paragraphs.0, 
            items
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sections::Section;
    use rstest::rstest;
use crate::tags::Tag;
use crate::blocks::Block;

    #[rstest]
    #[case(
        ["-> todo", 
            "", 
            "this is some text",
            "", 
            "[] alfa1", 
            "alfa2", 
            "", 
            "alfa3", 
            "alfa4", 
            "",
            "[x] bravo1", 
            "",
            "bravo2", 
            "",
            "bravo3", 
            "",
            "-> placeholder"].join("\n"),
        Ok(("\n\n-> placeholder", 
        Section::Todo {
            attrs: vec![],
            paragraphs: vec![
                            Block::Paragraph {
                                tags: vec![
    Tag::Text { text: "this is some text".to_string() }
                                ]
                            },
                ],
            items: vec![
                    Container::TodoItem { 
                        status: TodoStatus::NotDone,
                        paragraphs:
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

                    Container::TodoItem{
                        status: TodoStatus::Done,
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
    fn solo_test_todo(#[case] i: String, #[case] e: IResult<&str, Section>) {
        assert_eq!(e, todo(i.as_str()));
    }
}

