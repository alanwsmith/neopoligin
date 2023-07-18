use crate::blocks::Block;
use crate::containers::Container;
use crate::neo_section::neo_section;
use crate::section_attrs::SecAttr;
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
pub mod endsection;
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
pub mod section;
pub mod startcode;
pub mod textarea;
pub mod title;
pub mod todo;
pub mod vimeo;
pub mod warning;
pub mod youtube;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum AutoCapitalizeAttrOption {
    Off(),
    None(),
    On(),
    Sentences(),
    Words(),
    Characters(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum ContentEditableAttrOption {
    True(),
    False(),
    PlaintextOnly(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum DirAttrOption {
    Ltr(),
    Rtl(),
    Auto(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum HiddenAttrOption {
    Hidden(),
    UntilFound(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum InputModeAttrOption {
    None(),
    Text(),
    Decimal(),
    Numeric(),
    Tel(),
    Search(),
    Email(),
    Url(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum PopoverAttrOption {
    Auto(),
    Manual(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum TranslateAttrOption {
    Yes(),
    No(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum VirtualKeyboardPolicyAttrOption {
    Auto(),
    Manual(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum InputTypes {
    Button(),
    Checkbox(),
    Color(),
    Date(),
    DatatimeLocal(),
    Email(),
    File(),
    Hidden(),
    Image(),
    Month(),
    Number(),
    Password(),
    Radio(),
    Range(),
    Reset(),
    Search(),
    Submit(),
    Tel(),
    Text(),
    Time(),
    Url(),
    Week(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum Attribute {
    // these are global
    AccessKey(String),
    AutoCapitalize(AutoCapitalizeAttrOption),
    AutoFocus(),
    Class(Vec<String>),
    ContentEditable(ContentEditableAttrOption),
    Data(String, String),
    Dir(DirAttrOption),
    Draggable(bool),
    EnterKeyHint(String),
    Hidden(Option<HiddenAttrOption>),
    Id(String),
    Inert(),
    InputMode(InputModeAttrOption),
    Is(String),
    ItemId(String),
    ItemProp(String),
    ItemRef(String),
    ItemScope(String),
    ItemType(String),
    Lang(String),
    Nonce(String),
    Part(Vec<String>),
    Popover(PopoverAttrOption),
    PopoverTarget(String),
    Spellcheck(bool),
    Style(Vec<(String, String)>),
    TabIndex(i32), // can be negative
    Title(String),
    Translate(TranslateAttrOption),
    VirtualKeyboardPolicy(VirtualKeyboardPolicyAttrOption),
    // these are specific
    Accept(),
    AutoComplete(),
    Capture(),
    CrossOrigin(),
    Disabled(),
    ElementTiming(),
    For(),
    Max(),
    MaxLength(),
    Min(),
    MinLength(),
    Multiple(),
    Pattern(),
    ReadOnly(),
    Rel(),
    Required(),
    Size(),
    Step(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Section {
    AsideNewVersion {
        attributes: Vec<Attribute>,
        blocks: Vec<Block>,
    },
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
    SectionStart {
        attrs: Vec<SecAttr>,
    },
    SectionEnd,
    Textarea {
        attributes: Option<Vec<SecAttr>>,
        classes: Option<Vec<String>>,
        cols: Option<u32>,
        id: Option<String>,
        rows: Option<u32>,
        text: Option<String>,
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
    Warning {
        attrs: Vec<SecAttr>,
        paragraphs: Vec<Block>,
    },
    None,
}

pub fn neo_sections(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, results) = many0(neo_section)(source)?;
    Ok((source, results))
}

// TODO: Move this test into the spec json test

// #[cfg(test)]

// mod test {
//     use super::*;
//     use crate::blocks::Block;
//     use crate::neo_sections::Section;
//     use crate::section_attrs::SecAttr;
//     use crate::tag_attrs::TagAttr;
//     use crate::tags::Tag;
//     #[ignore]
//     #[test]
//     pub fn basic_integration() {
//         let lines = [
//             "-- title",
//             "-- class: alfa",
//             "",
//             "bravo charlie",
//             "delta echo",
//             "",
//             "foxtrot <golf",
//             "hotel",
//             "",
//             "whiskey <<tango|strong>> sierra",
//             "",
//             "-- h2",
//             "-- id: victor",
//             "",
//             "Crack the walnut.",
//             "Fasten <<two|strong>> pins.",
//             "",
//             "",
//             "",
//             "<<Guess the|abbr>> <<results|em|id: tango>>.",
//             "Hoist <<it|s>> up.",
//             "",
//             "<<Heave|sub>><<the|sup>><<line|span|class: alfa bravo charlie|id: delta>>",
//             "<<Take it away|q>>",
//             "",
//             "-- h3",
//             "",
//             "lift the hammer",
//             "",
//             "cap the jar",
//             "<<echo|link|https://www.example.com/|id: victor>>",
//             "",
//             "-- aside",
//             "",
//             "Add salt before you fry the egg",
//             "",
//             "-- blockquote",
//             "",
//             "Hang tinsel from both branches",
//             "",
//             "-- note",
//             "",
//             "alfa tango echo",
//             "",
//             "-- startcode",
//             "-- rust",
//             "-- id: delta",
//             "",
//             "-- h2",
//             "",
//             "That h2 should be in code",
//             "",
//             "-- endcode",
//         ]
//         .join("\n");
//         let expected = vec![
//             Section::Title {
//                 attrs: vec![SecAttr::Class(vec!["alfa".to_string()])],
//                 headline: Block::Headline {
//                     tags: vec![Tag::Text {
//                         text: "bravo charlie delta echo".to_string(),
//                     }],
//                 },
//                 paragraphs: vec![
//                     Block::Paragraph {
//                         tags: vec![
//                             Tag::Text {
//                                 text: "foxtrot ".to_string(),
//                             },
//                             Tag::LessThan {
//                                 text: "<g".to_string(),
//                             },
//                             Tag::Text {
//                                 text: "olf hotel".to_string(),
//                             },
//                         ],
//                     },
//                     Block::Paragraph {
//                         tags: vec![
//                             Tag::Text {
//                                 text: "whiskey ".to_string(),
//                             },
//                             Tag::Strong {
//                                 attrs: vec![],
//                                 text: "tango".to_string(),
//                             },
//                             Tag::Text {
//                                 text: " sierra".to_string(),
//                             },
//                         ],
//                     },
//                 ],
//             },
//             Section::H2 {
//                 attrs: vec![SecAttr::Id("victor".to_string())],
//                 headline: Block::Headline {
//                     tags: vec![
//                         Tag::Text {
//                             text: "Crack the walnut. Fasten ".to_string(),
//                         },
//                         Tag::Strong {
//                             attrs: vec![],
//                             text: "two".to_string(),
//                         },
//                         Tag::Text {
//                             text: " pins.".to_string(),
//                         },
//                     ],
//                 },
//                 paragraphs: vec![
//                     Block::Paragraph {
//                         tags: vec![
//                             Tag::Abbr {
//                                 attrs: vec![],
//                                 text: "Guess the".to_string(),
//                             },
//                             Tag::Text {
//                                 text: " ".to_string(),
//                             },
//                             Tag::Em {
//                                 attrs: vec![TagAttr::Id("tango".to_string())],
//                                 text: "results".to_string(),
//                             },
//                             Tag::Text {
//                                 text: ". Hoist ".to_string(),
//                             },
//                             Tag::S {
//                                 attrs: vec![],
//                                 text: "it".to_string(),
//                             },
//                             Tag::Text {
//                                 text: " up.".to_string(),
//                             },
//                         ],
//                     },
//                     Block::Paragraph {
//                         tags: vec![
//                             Tag::Sub {
//                                 attrs: vec![],
//                                 text: "Heave".to_string(),
//                             },
//                             Tag::Sup {
//                                 attrs: vec![],
//                                 text: "the".to_string(),
//                             },
//                             Tag::Span {
//                                 attrs: vec![
//                                     TagAttr::Class(vec![
//                                         "alfa".to_string(),
//                                         "bravo".to_string(),
//                                         "charlie".to_string(),
//                                     ]),
//                                     TagAttr::Id("delta".to_string()),
//                                 ],
//                                 text: "line".to_string(),
//                             },
//                             Tag::Text {
//                                 text: " ".to_string(),
//                             },
//                             Tag::Q {
//                                 attrs: vec![],
//                                 text: "Take it away".to_string(),
//                             },
//                         ],
//                     },
//                 ],
//             },
//             Section::H3 {
//                 attrs: vec![],
//                 headline: Block::Headline {
//                     tags: vec![Tag::Text {
//                         text: "lift the hammer".to_string(),
//                     }],
//                 },
//                 paragraphs: vec![Block::Paragraph {
//                     tags: vec![
//                         Tag::Text {
//                             text: "cap the jar ".to_string(),
//                         },
//                         Tag::Link {
//                             attrs: vec![TagAttr::Id("victor".to_string())],
//                             text: "echo".to_string(),
//                             url: "https://www.example.com/".to_string(),
//                         },
//                     ],
//                 }],
//             },
//             Section::Aside {
//                 attrs: vec![],
//                 paragraphs: vec![Block::Paragraph {
//                     tags: vec![Tag::Text {
//                         text: "Add salt before you fry the egg".to_string(),
//                     }],
//                 }],
//             },
//             Section::Blockquote {
//                 attrs: vec![],
//                 paragraphs: vec![Block::Paragraph {
//                     tags: vec![Tag::Text {
//                         text: "Hang tinsel from both branches".to_string(),
//                     }],
//                 }],
//             },
//             Section::Note {
//                 attrs: vec![],
//                 paragraphs: vec![Block::Paragraph {
//                     tags: vec![Tag::Text {
//                         text: "alfa tango echo".to_string(),
//                     }],
//                 }],
//             },
//             Section::Code {
//                 attrs: vec![
//                     SecAttr::Id("delta".to_string()),
//                     SecAttr::Class(vec!["language-rust".to_string()]),
//                 ],
//                 text: vec!["", "", "-- h2", "", "That h2 should be in code"].join("\n"),
//             },
//         ];
//         assert_eq!(expected, neo_sections(lines.as_str()).unwrap().1);
//     }
// }
