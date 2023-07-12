use crate::tag_attrs::TagAttr;
use crate::tags::abbr::abbr;
use crate::tags::b::b;
use crate::tags::code::code;
use crate::tags::dfn::dfn;
use crate::tags::em::em;
use crate::tags::i::i;
use crate::tags::img::img;
use crate::tags::kbd::kbd;
use crate::tags::less_than::less_than;
use crate::tags::link::link;
use crate::tags::mark::mark;
use crate::tags::q::q;
use crate::tags::s::s;
use crate::tags::samp::samp;
use crate::tags::small::small;
use crate::tags::span::span;
use crate::tags::strong::strong;
use crate::tags::sub::sub;
use crate::tags::sup::sup;
use crate::tags::text::text;
use crate::tags::u::u;
use crate::tags::var::var;
use crate::tags::wbr::wbr;
use nom::branch::alt;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;
use serde::Serialize;

pub mod abbr;
pub mod b;
pub mod basic;
pub mod code;
pub mod dfn;
pub mod em;
pub mod i;
pub mod img;
pub mod kbd;
pub mod less_than;
pub mod link;
pub mod mark;
pub mod q;
pub mod s;
pub mod samp;
pub mod small;
pub mod span;
pub mod strong;
pub mod sub;
pub mod sup;
pub mod text;
pub mod u;
pub mod var;
pub mod wbr;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Tag {
    Abbr {
        attrs: Vec<TagAttr>,
        text: String,
    },
    B {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Code {
        attrs: Vec<TagAttr>,
        // lang: Option<String>,
        text: String,
    },
    Dfn {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Em {
        attrs: Vec<TagAttr>,
        text: String,
    },
    I {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Img {
        attrs: Vec<TagAttr>,
        alt_text: Option<String>,
        src: String,
        caption: Option<String>,
    },
    Kbd {
        attrs: Vec<TagAttr>,
        text: String,
    },
    LessThan {
        text: String,
    },
    Link {
        attrs: Vec<TagAttr>,
        text: String,
        url: String,
    },
    Mark {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Q {
        attrs: Vec<TagAttr>,
        text: String,
    },
    S {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Samp {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Small {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Span {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Strong {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Sub {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Sup {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Text {
        text: String,
    },
    U {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Var {
        attrs: Vec<TagAttr>,
        text: String,
    },
    Wbr {
        attrs: Vec<TagAttr>,
        text: String,
    },
}

pub fn tags(source: &str) -> IResult<&str, Vec<Tag>> {
    let (source, snippets) = many_till(
        alt((
            alt((
                img, less_than, abbr, b, code, dfn, em, i, kbd, link, mark, q, s, samp,
            )),
            alt((small, span, strong, sub, sup, text, u, var, wbr)),
        )),
        eof,
    )(source)?;
    Ok((source, snippets.0))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn basic_text() {
        let line = "the quick brown fox";
        let expected = vec![Tag::Text {
            text: "the quick brown fox".to_string(),
        }];
        assert_eq!(expected, tags(line).unwrap().1);
    }

    #[test]
    pub fn text_with_strong() {
        let line = "alfa <<bravo|strong>> charlie";
        let expected = vec![
            Tag::Text {
                text: "alfa ".to_string(),
            },
            Tag::Strong {
                attrs: vec![],
                text: "bravo".to_string(),
            },
            Tag::Text {
                text: " charlie".to_string(),
            },
        ];
        assert_eq!(expected, tags(line).unwrap().1);
    }

    #[test]
    pub fn less_than() {
        let line = "delta <- <<echo|strong>> foxtrot";
        let expected = vec![
            Tag::Text {
                text: "delta ".to_string(),
            },
            Tag::LessThan {
                text: "<-".to_string(),
            },
            Tag::Text {
                text: " ".to_string(),
            },
            Tag::Strong {
                attrs: vec![],
                text: "echo".to_string(),
            },
            Tag::Text {
                text: " foxtrot".to_string(),
            },
        ];
        assert_eq!(expected, tags(line).unwrap().1);
    }
}
