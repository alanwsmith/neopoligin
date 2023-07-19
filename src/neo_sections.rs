use nom::branch::alt;
use nom::multi::many0;
use nom::IResult;
use serde::{Deserialize, Serialize};
pub mod aside;
use crate::neo_sections::aside::aside;
pub mod attributes;
use crate::neo_sections::attributes::attributes;
pub mod audio;
use crate::neo_sections::audio::audio;
pub mod blockquote;
use crate::neo_sections::blockquote::blockquote;
pub mod blurb;
use crate::neo_sections::blurb::blurb;
pub mod canvas;
use crate::neo_sections::canvas::canvas;
pub mod categories;
use crate::neo_sections::categories::categories;
pub mod checklist;
use crate::neo_sections::checklist::checklist;
pub mod code;
use crate::neo_sections::code::code;
pub mod css;
use crate::neo_sections::css::css;
pub mod endarticle;
use crate::neo_sections::endarticle::endarticle;
pub mod endcode;
use crate::neo_sections::endcode::endcode;
pub mod enddiv;
use crate::neo_sections::enddiv::enddiv;
pub mod endhtml;
use crate::neo_sections::endhtml::endhtml;
pub mod endneoexample;
use crate::neo_sections::endneoexample::endneoexample;
pub mod endsection;
use crate::neo_sections::endsection::endsection;
pub mod head;
use crate::neo_sections::head::head;
pub mod html;
use crate::neo_sections::html::html;
pub mod pre;
use crate::neo_sections::pre::pre;
pub mod script;
use crate::neo_sections::script::script;
pub mod startarticle;
use crate::neo_sections::startarticle::startarticle;
pub mod startcode;
use crate::neo_sections::startcode::startcode;
pub mod startdiv;
use crate::neo_sections::startdiv::startdiv;
pub mod starthtml;
use crate::neo_sections::starthtml::starthtml;
pub mod startneoexample;
use crate::neo_sections::startneoexample::startneoexample;
pub mod startsection;
use crate::neo_sections::startsection::startsection;
use crate::attrs::Attribute;
use crate::blocks::Block;
use crate::containers::Container;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NeoSection {
    Aside {
        attrs: Option<Vec<Attribute>>,
        blocks: Option<Vec<Block>>,
    },
    Attributes {
        attrs: Option<Vec<Attribute>>,
    },
    Audio {
        attrs: Option<Vec<Attribute>>,
        blocks: Option<Vec<Block>>,
    },
    Blockquote {
        attrs: Option<Vec<Attribute>>,
        blocks: Option<Vec<Block>>,
    },
    Blurb {
        attrs: Option<Vec<Attribute>>,
        blocks: Option<Vec<Block>>,
    },
    Canvas {
        attrs: Option<Vec<Attribute>>,
    },
    Categories {
        list: Vec<String>,
    },
    Checklist {
        attrs: Option<Vec<Attribute>>,
        blocks: Option<Vec<Block>>,
        items: Option<Vec<Container>>,
    },
    Code {
        attrs: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    CSS {
        text: Option<String>,
    },
    EndArticle,
    EndCode,
    EndDiv,
    EndHTML,
    EndNeoExample,
    EndSection,
    Head {
        text: Option<String>,
    },
    HTML {
        text: Option<String>,
    },
    Pre {
        text: Option<String>,
    },
    Script {
        text: Option<String>,
    },
    StartArticle,
    StartCode,
    StartDiv,
    StartHTML,
    StartNeoExample,
    StartSection,
    None,
}

// this is split out from neo_section to enable
// direct testing of individual sections
pub fn neo_sections(source: &str) -> IResult<&str, Vec<NeoSection>> {
    let (source, results) = many0(neo_section)(source)?;
    Ok((source, results))
}

pub fn neo_section(source: &str) -> IResult<&str, NeoSection> {
    let (source, results) = alt((
        alt((
            aside,
            attributes,
            audio,
            blockquote,
            blurb,
            canvas,
            categories,
            checklist,
            code,
            css,
            endarticle,
            endcode,
            enddiv,
            endhtml,
            endneoexample,
            endsection,
            head,
        )),
        alt((
            html,
            pre,
            script, 
            startarticle,
            startcode,
            startdiv,
            starthtml,
            startneoexample,
            startsection,
        ))
    ))(source)?;
    Ok((source, results))
}
