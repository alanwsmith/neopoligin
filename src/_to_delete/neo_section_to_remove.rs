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
pub mod h1;
use crate::neo_sections::h1::h1;
pub mod h2;
use crate::neo_sections::h2::h2;
pub mod h3;
use crate::neo_sections::h3::h3;
pub mod h4;
use crate::neo_sections::h4::h4;
pub mod h5;
use crate::neo_sections::h5::h5;
pub mod h6;
use crate::neo_sections::h6::h6;
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
pub mod data;
use crate::neo_sections::data::data;
pub mod hidden;
use crate::neo_sections::hidden::hidden;
pub mod hr;
use crate::neo_sections::hr::hr;
pub mod image;
use crate::neo_sections::image::image;
pub mod images;
use crate::neo_sections::images::images;
pub mod list;
use crate::neo_sections::list::list;
pub mod menu;
use crate::neo_sections::menu::menu;
pub mod nav;
use crate::neo_sections::nav::nav;
pub mod note;
use crate::neo_sections::note::note;
pub mod notes;
use crate::neo_sections::notes::notes;
pub mod olist;
use crate::neo_sections::olist::olist;
pub mod p;
use crate::neo_sections::p::p;
pub mod reference;
use crate::neo_sections::reference::reference;
pub mod results;
use crate::neo_sections::results::results;
pub mod subtitle;
use crate::neo_sections::subtitle::subtitle;
pub mod table;
use crate::neo_sections::table::table;
pub mod textarea;
use crate::neo_sections::textarea::textarea;
pub mod title;
use crate::neo_sections::title::title;
pub mod todo;
use crate::neo_sections::todo::todo;
pub mod vimeo;
use crate::neo_sections::vimeo::vimeo;
pub mod warning;
use crate::neo_sections::warning::warning;
pub mod widget;
use crate::neo_sections::widget::widget;
pub mod youtube;
use crate::neo_sections::youtube::youtube;
pub mod config;
use crate::neo_sections::config::config;
pub mod details;
use crate::neo_sections::details::details;
pub mod dlist;
use crate::neo_sections::dlist::dlist;
pub mod ext;
use crate::neo_sections::ext::ext;
pub mod include;
use crate::neo_sections::include::include;
pub mod object;
use crate::neo_sections::object::object;

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
    H1 {
        attrs: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    H2 {
        attrs: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    H3 {
        attrs: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    H4 {
        attrs: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    H5 {
        attrs: Option<Vec<Attribute>>,
        text: Option<String>,
    },
    H6 {
        attrs: Option<Vec<Attribute>>,
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
    Title {
        attrs: Option<Vec<Attribute>>,
        blocks: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    None,
    Placeholder {
        attrs: Option<Vec<Attribute>>,
        text: Option<String>,
    },
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
            data,
            h1,
            h2,
            h3,
            h4,
            h5,
            h6,
            html,
            pre,
            script,
            startarticle,
            startcode,
            startdiv,
            starthtml,
            startneoexample,
            startsection,
        )),
        alt((data, hidden, hr, images, image, list, menu, nav, reference)),
        alt((
            notes, note, olist, p, results, subtitle, table, textarea, title, todo, vimeo, warning,
            widget, youtube, config, details, dlist, ext, include, object,
        )),
    ))(source)?;
    Ok((source, results))
}
