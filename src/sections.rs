use crate::blocks::Block;
use crate::containers::Container;
use crate::section_attrs::SecAttr;
use crate::sections::aside::aside;
use crate::sections::blockquote::blockquote;
use crate::sections::checklist::checklist;
use crate::sections::closediv::closediv;
use crate::sections::code::code;
use crate::sections::css::css;
use crate::sections::endcode::endcode;
use crate::sections::h::h;
use crate::sections::hidden::hidden;
use crate::sections::hr::hr;
use crate::sections::html::html;
use crate::sections::image::image;
use crate::sections::list::list;
use crate::sections::note::note;
use crate::sections::notes::notes;
use crate::sections::olist::olist;
use crate::sections::opendiv::opendiv;
use crate::sections::p::p;
use crate::sections::pre::pre;
use crate::sections::script::script;
use crate::sections::startcode::startcode;
use crate::sections::title::title;
use crate::sections::todo::todo;
use crate::sections::vimeo::vimeo;
use crate::sections::youtube::youtube;
use nom::branch::alt;
use nom::multi::many0;
use nom::IResult;
use serde::{Deserialize, Serialize};

pub mod aside;
pub mod blockquote;
pub mod checklist;
pub mod closediv;
pub mod code;
pub mod css;
pub mod endcode;
pub mod h;
pub mod hidden;
pub mod hr;
pub mod html;
pub mod image;
pub mod list;
pub mod note;
pub mod notes;
pub mod olist;
pub mod opendiv;
pub mod p;
pub mod pre;
pub mod script;
pub mod startcode;
pub mod title;
pub mod todo;
pub mod vimeo;
pub mod youtube;

// #[derive(Debug, PartialEq)]
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Section {
    Aside {
        attrs: Vec<SecAttr>,
        paragraphs: Vec<Block>,
    },
    Blockquote {
        attrs: Vec<SecAttr>,
        paragraphs: Vec<Block>,
    },
    Checklist {
        attrs: Vec<SecAttr>,
        items: Vec<Container>,
        paragraphs: Vec<Block>,
    },
    CloseDiv,
    Code {
        attrs: Vec<SecAttr>,
        text: String,
    },
    CSS {
        text: String,
    },
    H1 {
        attrs: Vec<SecAttr>,
        headline: Block,
        paragraphs: Vec<Block>,
    },
    H2 {
        attrs: Vec<SecAttr>,
        headline: Block,
        paragraphs: Vec<Block>,
    },
    H3 {
        attrs: Vec<SecAttr>,
        headline: Block,
        paragraphs: Vec<Block>,
    },
    H4 {
        attrs: Vec<SecAttr>,
        headline: Block,
        paragraphs: Vec<Block>,
    },
    H5 {
        attrs: Vec<SecAttr>,
        headline: Block,
        paragraphs: Vec<Block>,
    },
    H6 {
        attrs: Vec<SecAttr>,
        headline: Block,
        paragraphs: Vec<Block>,
    },
    Hidden,
    Html {
        text: String,
    },
    Hr {
        attrs: Vec<SecAttr>,
    },
    Image {
        alt: String,
        attrs: Vec<SecAttr>,
        src: String,
    },
    List {
        attrs: Vec<SecAttr>,
        items: Vec<Container>,
        paragraphs: Vec<Block>,
    },
    Note {
        attrs: Vec<SecAttr>,
        paragraphs: Vec<Block>,
    },
    Notes {
        attrs: Vec<SecAttr>,
        items: Vec<Container>,
        paragraphs: Vec<Block>,
    },
    OList {
        attrs: Vec<SecAttr>,
        items: Vec<Container>,
        paragraphs: Vec<Block>,
    },
    OpenDiv {
        attrs: Vec<SecAttr>,
    },
    P {
        attrs: Vec<SecAttr>,
        paragraphs: Vec<Block>,
    },
    Pre {
        attrs: Vec<SecAttr>,
        text: String,
    },
    Script {
        text: String,
    },
    Title {
        attrs: Vec<SecAttr>,
        headline: Block,
        paragraphs: Vec<Block>,
    },
    Todo {
        attrs: Vec<SecAttr>,
        items: Vec<Container>,
        paragraphs: Vec<Block>,
    },
    Youtube {
        attrs: Vec<SecAttr>,
        id: String,
        paragraphs: Vec<Block>,
    },
    Vimeo {
        attrs: Vec<SecAttr>,
        id: String,
        paragraphs: Vec<Block>,
    },
    None,
}

pub fn sections(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, results) = many0(alt((
        // order matters here so things don't get flipped
        alt((title, hidden, html, h)),
        alt((pre, p)),
        alt((code, list)),
        alt((notes, note, checklist)),
        alt((
            aside, blockquote, closediv, code, css, endcode, hr, image, opendiv, olist, pre,
            script, startcode, todo, vimeo, youtube,
        )),
    )))(source)?;
    Ok((source, results))
}

#[cfg(test)]

mod test {
    use super::*;
    use crate::blocks::Block;
    use crate::section_attrs::SecAttr;
    use crate::sections::Section;
    use crate::tag_attrs::TagAttr;
    use crate::tags::Tag;

    #[ignore]
    #[test]
    pub fn basic_integration() {
        let lines = [
            "-- title",
            "-- class: alfa",
            "",
            "bravo charlie",
            "delta echo",
            "",
            "foxtrot <golf",
            "hotel",
            "",
            "whiskey <<tango|strong>> sierra",
            "",
            "-- h2",
            "-- id: victor",
            "",
            "Crack the walnut.",
            "Fasten <<two|strong>> pins.",
            "",
            "",
            "",
            "<<Guess the|abbr>> <<results|em|id: tango>>.",
            "Hoist <<it|s>> up.",
            "",
            "<<Heave|sub>><<the|sup>><<line|span|class: alfa bravo charlie|id: delta>>",
            "<<Take it away|q>>",
            "",
            "-- h3",
            "",
            "lift the hammer",
            "",
            "cap the jar",
            "<<echo|link|https://www.example.com/|id: victor>>",
            "",
            "-- aside",
            "",
            "Add salt before you fry the egg",
            "",
            "-- blockquote",
            "",
            "Hang tinsel from both branches",
            "",
            "-- note",
            "",
            "alfa tango echo",
            "",
            "-- startcode",
            "-- rust",
            "-- id: delta",
            "",
            "-- h2",
            "",
            "That h2 should be in code",
            "",
            "-- endcode",
        ]
        .join("\n");
        let expected = vec![
            Section::Title {
                attrs: vec![SecAttr::Class(vec!["alfa".to_string()])],
                headline: Block::Headline {
                    tags: vec![Tag::Text {
                        text: "bravo charlie delta echo".to_string(),
                    }],
                },
                paragraphs: vec![
                    Block::Paragraph {
                        tags: vec![
                            Tag::Text {
                                text: "foxtrot ".to_string(),
                            },
                            Tag::LessThan {
                                text: "<g".to_string(),
                            },
                            Tag::Text {
                                text: "olf hotel".to_string(),
                            },
                        ],
                    },
                    Block::Paragraph {
                        tags: vec![
                            Tag::Text {
                                text: "whiskey ".to_string(),
                            },
                            Tag::Strong {
                                attrs: vec![],
                                text: "tango".to_string(),
                            },
                            Tag::Text {
                                text: " sierra".to_string(),
                            },
                        ],
                    },
                ],
            },
            Section::H2 {
                attrs: vec![SecAttr::Id("victor".to_string())],
                headline: Block::Headline {
                    tags: vec![
                        Tag::Text {
                            text: "Crack the walnut. Fasten ".to_string(),
                        },
                        Tag::Strong {
                            attrs: vec![],
                            text: "two".to_string(),
                        },
                        Tag::Text {
                            text: " pins.".to_string(),
                        },
                    ],
                },
                paragraphs: vec![
                    Block::Paragraph {
                        tags: vec![
                            Tag::Abbr {
                                attrs: vec![],
                                text: "Guess the".to_string(),
                            },
                            Tag::Text {
                                text: " ".to_string(),
                            },
                            Tag::Em {
                                attrs: vec![TagAttr::Id("tango".to_string())],
                                text: "results".to_string(),
                            },
                            Tag::Text {
                                text: ". Hoist ".to_string(),
                            },
                            Tag::S {
                                attrs: vec![],
                                text: "it".to_string(),
                            },
                            Tag::Text {
                                text: " up.".to_string(),
                            },
                        ],
                    },
                    Block::Paragraph {
                        tags: vec![
                            Tag::Sub {
                                attrs: vec![],
                                text: "Heave".to_string(),
                            },
                            Tag::Sup {
                                attrs: vec![],
                                text: "the".to_string(),
                            },
                            Tag::Span {
                                attrs: vec![
                                    TagAttr::Class(vec![
                                        "alfa".to_string(),
                                        "bravo".to_string(),
                                        "charlie".to_string(),
                                    ]),
                                    TagAttr::Id("delta".to_string()),
                                ],
                                text: "line".to_string(),
                            },
                            Tag::Text {
                                text: " ".to_string(),
                            },
                            Tag::Q {
                                attrs: vec![],
                                text: "Take it away".to_string(),
                            },
                        ],
                    },
                ],
            },
            Section::H3 {
                attrs: vec![],
                headline: Block::Headline {
                    tags: vec![Tag::Text {
                        text: "lift the hammer".to_string(),
                    }],
                },
                paragraphs: vec![Block::Paragraph {
                    tags: vec![
                        Tag::Text {
                            text: "cap the jar ".to_string(),
                        },
                        Tag::Link {
                            attrs: vec![TagAttr::Id("victor".to_string())],
                            text: "echo".to_string(),
                            url: "https://www.example.com/".to_string(),
                        },
                    ],
                }],
            },
            Section::Aside {
                attrs: vec![],
                paragraphs: vec![Block::Paragraph {
                    tags: vec![Tag::Text {
                        text: "Add salt before you fry the egg".to_string(),
                    }],
                }],
            },
            Section::Blockquote {
                attrs: vec![],
                paragraphs: vec![Block::Paragraph {
                    tags: vec![Tag::Text {
                        text: "Hang tinsel from both branches".to_string(),
                    }],
                }],
            },
            Section::Note {
                attrs: vec![],
                paragraphs: vec![Block::Paragraph {
                    tags: vec![Tag::Text {
                        text: "alfa tango echo".to_string(),
                    }],
                }],
            },
            Section::Code {
                attrs: vec![
                    SecAttr::Id("delta".to_string()),
                    SecAttr::Class(vec!["language-rust".to_string()]),
                ],
                text: vec!["", "", "-- h2", "", "That h2 should be in code"].join("\n"),
            },
        ];
        assert_eq!(expected, sections(lines.as_str()).unwrap().1);
    }
}
