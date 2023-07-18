use crate::snippets::text::text;
use nom::branch::alt;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;
use serde::{Deserialize, Serialize};

pub mod text;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Snippet {
    Text {
        text: String,
    },
}

pub fn snippets(source: &str) -> IResult<&str, Vec<Snippet>> {
    let (source, snippets) = many_till(
        alt((
            text, text
            // alt((
            //     img, less_than, abbr, b, code, dfn, em, i, kbd, link, mark, q, s, samp,
            // )),
            // alt((small, span, strong, sub, sup, text, u, var, wbr)),
        )),
        eof,
    )(source)?;
    Ok((source, snippets.0))
}



// // use crate::tag_attrs::TagAttr;
// // use crate::snippets::abbr::abbr;
// // use crate::snippets::b::b;
// // use crate::snippets::code::code;
// // use crate::snippets::dfn::dfn;
// // use crate::snippets::em::em;
// // use crate::snippets::i::i;
// // use crate::snippets::img::img;
// // use crate::snippets::kbd::kbd;
// // use crate::snippets::less_than::less_than;
// // use crate::snippets::link::link;
// // use crate::snippets::mark::mark;
// // use crate::snippets::q::q;
// // use crate::snippets::s::s;
// // use crate::snippets::samp::samp;
// // use crate::snippets::small::small;
// // use crate::snippets::span::span;
// // use crate::snippets::strong::strong;
// // use crate::snippets::sub::sub;
// // use crate::snippets::sup::sup;
// use crate::snippets::text::text;
// // use crate::snippets::u::u;
// // use crate::snippets::var::var;
// // use crate::snippets::wbr::wbr;
// use nom::branch::alt;
// use nom::combinator::eof;
// use nom::multi::many_till;
// use nom::IResult;
// use serde::{Deserialize, Serialize};

// pub mod abbr;
// pub mod b;
// pub mod basic;
// pub mod code;
// pub mod dfn;
// pub mod em;
// pub mod i;
// pub mod img;
// pub mod kbd;
// pub mod less_than;
// pub mod link;
// pub mod mark;
// pub mod q;
// pub mod s;
// pub mod samp;
// pub mod small;
// pub mod span;
// pub mod strong;
// pub mod sub;
// pub mod sup;
// pub mod text;
// pub mod u;
// pub mod var;
// pub mod wbr;

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", rename_all = "lowercase")]
// pub enum Snippet {
//     Text {
//         text: String,
//     },
//     // Abbr {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // B {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // Code {
//     //     attrs: Vec<TagAttr>,
//     //     // lang: Option<String>,
//     //     text: String,
//     // },
//     // Dfn {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // Em {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // I {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // Img {
//     //     attrs: Vec<TagAttr>,
//     //     alt_text: Option<String>,
//     //     src: String,
//     //     caption: Option<String>,
//     // },
//     // Kbd {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // LessThan {
//     //     text: String,
//     // },
//     // Link {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     //     url: String,
//     // },
//     // Mark {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // Q {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // S {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // Samp {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // Small {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // Span {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // Strong {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // Sub {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // Sup {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },

//     // U {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // Var {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
//     // Wbr {
//     //     attrs: Vec<TagAttr>,
//     //     text: String,
//     // },
// }

// pub fn tags(source: &str) -> IResult<&str, Vec<Snippet>> {
//     let (source, snippets) = many_till(
//         alt((
//             text, text
//             // alt((
//             //     img, less_than, abbr, b, code, dfn, em, i, kbd, link, mark, q, s, samp,
//             // )),
//             // alt((small, span, strong, sub, sup, text, u, var, wbr)),
//         )),
//         eof,
//     )(source)?;
//     Ok((source, snippets.0))
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     pub fn basic_text() {
//         let line = "the quick brown fox";
//         let expected = vec![Tag::Text {
//             text: "the quick brown fox".to_string(),
//         }];
//         assert_eq!(expected, tags(line).unwrap().1);
//     }

//     #[test]
//     pub fn text_with_strong() {
//         let line = "alfa <<bravo|strong>> charlie";
//         let expected = vec![
//             Tag::Text {
//                 text: "alfa ".to_string(),
//             },
//             Tag::Strong {
//                 attrs: vec![],
//                 text: "bravo".to_string(),
//             },
//             Tag::Text {
//                 text: " charlie".to_string(),
//             },
//         ];
//         assert_eq!(expected, tags(line).unwrap().1);
//     }

//     #[test]
//     pub fn less_than() {
//         let line = "delta <- <<echo|strong>> foxtrot";
//         let expected = vec![
//             Tag::Text {
//                 text: "delta ".to_string(),
//             },
//             Tag::LessThan {
//                 text: "<-".to_string(),
//             },
//             Tag::Text {
//                 text: " ".to_string(),
//             },
//             Tag::Strong {
//                 attrs: vec![],
//                 text: "echo".to_string(),
//             },
//             Tag::Text {
//                 text: " foxtrot".to_string(),
//             },
//         ];
//         assert_eq!(expected, tags(line).unwrap().1);
//     }
// }
