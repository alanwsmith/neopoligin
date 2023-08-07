use crate::attributes::*;
use crate::blocks::Block;
use crate::containers::Container;
use crate::neo_sections::aside_section::aside_section;
use crate::neo_sections::blockquote_section::blockquote_section;
use crate::neo_sections::bookmark_section::bookmark_section;
use crate::neo_sections::categories_section::categories_section;
use crate::neo_sections::checklist_section::checklist_section;
use crate::neo_sections::code_section::code_section;
use crate::neo_sections::endarticle_section::endarticle_section;
use crate::neo_sections::endcode_section::endcode_section;
use crate::neo_sections::endcss_section::endcss_section;
use crate::neo_sections::enddiv_section::enddiv_section;
use crate::neo_sections::endhtml_section::endhtml_section;
use crate::neo_sections::endpre_section::endpre_section;
use crate::neo_sections::endresults_section::endresults_section;
use crate::neo_sections::endsection_section::endsection_section;
use crate::neo_sections::endscript_section::endscript_section;
use crate::neo_sections::endtldr_section::endtldr_section;
use crate::neo_sections::h1_section::h1_section;
use crate::neo_sections::h2_section::h2_section;
use crate::neo_sections::h3_section::h3_section;
use crate::neo_sections::h4_section::h4_section;
use crate::neo_sections::h5_section::h5_section;
use crate::neo_sections::h6_section::h6_section;
use crate::neo_sections::hidden_section::hidden_section;
use crate::neo_sections::html_section::html_section;
use crate::neo_sections::image_section::image_section;
use crate::neo_sections::list_section::list_section;
use crate::neo_sections::metadata_section::*;
use crate::neo_sections::nav_section::nav_section;
use crate::neo_sections::note_section::note_section;
use crate::neo_sections::notes_section::notes_section;
use crate::neo_sections::olist_section::olist_section;
use crate::neo_sections::p_section::p_section;
use crate::neo_sections::pre_section::pre_section;
use crate::neo_sections::script_section::script_section;
use crate::neo_sections::startarticle_section::startarticle_section;
use crate::neo_sections::startcode_section::startcode_section;
use crate::neo_sections::startcss_section::startcss_section;
use crate::neo_sections::startdiv_section::startdiv_section;
use crate::neo_sections::starthtml_section::starthtml_section;
use crate::neo_sections::startpre_section::startpre_section;
use crate::neo_sections::startresults_section::startresults_section;
use crate::neo_sections::startscript_section::startscript_section;
use crate::neo_sections::startsection_section::startsection_section;
use crate::neo_sections::starttldr_section::starttldr_section;
use crate::neo_sections::subtitle_section::subtitle_section;
use crate::neo_sections::title_section::title_section;
use crate::neo_sections::todo_section::todo_section;
use crate::neo_sections::vimeo_section::vimeo_section;
use crate::neo_sections::warning_section::warning_section;
use crate::neo_sections::warnings_section::warnings_section;
use crate::neo_sections::youtube_section::youtube_section;
use nom::error::VerboseError;
use nom::IResult;
use nom::{branch::alt, character::complete::multispace0};
use serde::{Deserialize, Serialize};

pub mod aside_section;
pub mod bookmark_section;
pub mod blockquote_section;
pub mod categories_section;
pub mod checklist_section;
pub mod code_section;
// pub mod css_section;
pub mod endarticle_section;
pub mod endcode_section;
pub mod endcss_section;
pub mod enddiv_section;
pub mod endhtml_section;
pub mod endpre_section;
pub mod endresults_section;
pub mod endscript_section;
pub mod endsection_section;
pub mod endtldr_section;
pub mod h1_section;
pub mod h2_section;
pub mod h3_section;
pub mod h4_section;
pub mod h5_section;
pub mod h6_section;
pub mod hidden_section;
pub mod html_section;
pub mod image_section;
pub mod list_section;
pub mod metadata_section;
pub mod nav_section;
pub mod note_section;
pub mod notes_section;
pub mod olist_section;
pub mod p_section;
pub mod pre_section;
pub mod script_section;
pub mod startarticle_section;
pub mod startcode_section;
pub mod startcss_section;
pub mod startdiv_section;
pub mod starthtml_section;
pub mod startpre_section;
pub mod startresults_section;
pub mod startscript_section;
pub mod startsection_section;
pub mod starttldr_section;
pub mod subtitle_section;
pub mod title_section;
pub mod todo_section;
pub mod vimeo_section;
pub mod warning_section;
pub mod warnings_section;
pub mod youtube_section;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NeoSection {
    Aside {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    Blockquote {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    Bookmark {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    Categories {
        list: Option<Vec<String>>
    },
    Checklist {
        attributes: Option<Vec<AttributeV2>>,
        items: Option<Vec<Container>>,
        preface: Option<Vec<Block>>,
    },
    Code {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<String>,
    },
    Css {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<String>,
    },
    EndArticle {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    EndCss {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    EndDiv {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    EndHtml {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    EndPre {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    EndResults {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    EndScript {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    EndSection {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    EndTlDr {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    H1 {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    H2 {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    H3 {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    H4 {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    H5 {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    H6 {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    Hidden {},
    Html {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<String>,
    },
    Image {
        attributes: Option<Vec<AttributeV2>>,
        name: Option<String>,
        src: Option<String>,
    },
    List {
        attributes: Option<Vec<AttributeV2>>,
        items: Option<Vec<Container>>,
        preface: Option<Vec<Block>>,
    },
    Nav {
        attributes: Option<Vec<AttributeV2>>,
        items: Option<Vec<Container>>,
    },
    Note {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    Notes {
        attributes: Option<Vec<AttributeV2>>,
        items: Option<Vec<Container>>,
        preface: Option<Vec<Block>>,
    },
    Metadata {
        list: Option<Vec<MetadataItem>>,
    },
    Olist {
        attributes: Option<Vec<AttributeV2>>,
        items: Option<Vec<Container>>,
        preface: Option<Vec<Block>>,
    },
    P {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    Pre {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<String>,
    },
    Results {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<String>,
    },
    Script {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<String>,
    },
    StartArticle {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    }, 
    StartCode {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<String>,
    },
    StartDiv {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    }, 
    StartSection {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    }, 
    StartTlDr {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    }, 
    Subtitle {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    Title {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    Todo {
        attributes: Option<Vec<AttributeV2>>,
        items: Option<Vec<Container>>,
        preface: Option<Vec<Block>>,
    },
    Vimeo {
        attributes: Option<Vec<AttributeV2>>,
        id: Option<String>,
    },
    Warning {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    Warnings {
        attributes: Option<Vec<AttributeV2>>,
        items: Option<Vec<Container>>,
        preface: Option<Vec<Block>>,
    },
    Youtube {
        attributes: Option<Vec<AttributeV2>>,
        id: Option<String>,
    },
}

pub fn neo_section(source: &str) -> IResult<&str, NeoSection, VerboseError<&str>> {
    // dbg!(&source);
    let (source, _) = multispace0(source)?;
    // let (source, section) = p_section(source)?;
    // dbg!(&source);
    let (source, section) = alt((
        alt((
            startsection_section,
            aside_section,
            blockquote_section,
            checklist_section,
            html_section,
        image_section,
        list_section,
        olist_section,
        notes_section,
        note_section,
        //     metadata_section,
        startcode_section,
        startcss_section,
        startarticle_section,
        startdiv_section,
        starthtml_section,
        startpre_section,
        startscript_section,
        pre_section,
    )),

    alt((
        endarticle_section,
        endcode_section,
        endcss_section,
        enddiv_section,
        endhtml_section,
        endpre_section,
        endresults_section,
        endscript_section,
        endsection_section,
        startresults_section,
        starttldr_section,
        endtldr_section,
        vimeo_section,
        warning_section,
        warnings_section,
        youtube_section,
        todo_section,
    )), 
    alt ((
        code_section,
        // css_section,
        h1_section,
        h2_section,
        h3_section,
        h4_section,
        h5_section,
        h6_section,
        script_section,
        metadata_section,
        title_section,
        pre_section,
        subtitle_section,
        nav_section,
        categories_section,
        hidden_section,
        bookmark_section,
        p_section,
    ))
    ))(source)?;
    Ok((source, section))
}
