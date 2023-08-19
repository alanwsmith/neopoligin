use crate::attributes::*;
use crate::blocks::Block;
use crate::containers::Container;
use crate::neo_sections::articleend_section::articleend_section;
use crate::neo_sections::articlestart_section::articlestart_section;
use crate::neo_sections::aside_section::aside_section;
use crate::neo_sections::blockquote_section::blockquote_section;
use crate::neo_sections::bookmark_section::bookmark_section;
use crate::neo_sections::categories_section::categories_section;
use crate::neo_sections::checklist_section::checklist_section;
use crate::neo_sections::code_section::code_section;
use crate::neo_sections::codeend_section::codeend_section;
use crate::neo_sections::codestart_section::codestart_section;
use crate::neo_sections::comment_section::comment_section;
use crate::neo_sections::css_section::css_section;
use crate::neo_sections::cssend_section::cssend_section;
use crate::neo_sections::cssstart_section::cssstart_section;
use crate::neo_sections::divend_section::divend_section;
use crate::neo_sections::divstart_section::divstart_section;
use crate::neo_sections::groups_section::groups_section;
use crate::neo_sections::h1_section::h1_section;
use crate::neo_sections::h2_section::h2_section;
use crate::neo_sections::h3_section::h3_section;
use crate::neo_sections::h4_section::h4_section;
use crate::neo_sections::h5_section::h5_section;
use crate::neo_sections::h6_section::h6_section;
use crate::neo_sections::hr_section::hr_section;
use crate::neo_sections::html_section::html_section;
use crate::neo_sections::htmlend_section::htmlend_section;
use crate::neo_sections::htmlstart_section::htmlstart_section;
use crate::neo_sections::image_section::image_section;
use crate::neo_sections::list_section::list_section;
use crate::neo_sections::metadata_section::*;
use crate::neo_sections::nav_section::nav_section;
use crate::neo_sections::note_section::note_section;
use crate::neo_sections::notes_section::notes_section;
use crate::neo_sections::olist_section::olist_section;
use crate::neo_sections::p_section::p_section;
use crate::neo_sections::pre_section::pre_section;
use crate::neo_sections::preend_section::preend_section;
use crate::neo_sections::prestart_section::prestart_section;
use crate::neo_sections::ref_section::ref_section;
use crate::neo_sections::results_section::results_section;
use crate::neo_sections::resultsend_section::resultsend_section;
use crate::neo_sections::resultsstart_section::resultsstart_section;
use crate::neo_sections::script_section::script_section;
use crate::neo_sections::scriptend_section::scriptend_section;
use crate::neo_sections::scriptstart_section::scriptstart_section;
use crate::neo_sections::sectionend_section::sectionend_section;
use crate::neo_sections::sectionstart_section::sectionstart_section;
use crate::neo_sections::subtitle_section::subtitle_section;
use crate::neo_sections::title_section::title_section;
use crate::neo_sections::tldrend_section::tldrend_section;
use crate::neo_sections::tldrstart_section::tldrstart_section;
use crate::neo_sections::todo_section::todo_section;
use crate::neo_sections::vimeo_section::vimeo_section;
use crate::neo_sections::warning_section::warning_section;
use crate::neo_sections::warnings_section::warnings_section;
use crate::neo_sections::youtube_section::youtube_section;
use nom::error::VerboseError;
use nom::IResult;
use nom::{branch::alt, character::complete::multispace0};
use serde::{Deserialize, Serialize};

pub mod articleend_section;
pub mod articlestart_section;
pub mod aside_section;
pub mod blockquote_section;
pub mod bookmark_section;
pub mod categories_section;
pub mod checklist_section;
pub mod code_section;
pub mod codeend_section;
pub mod codestart_section;
pub mod comment_section;
pub mod css_section;
pub mod cssend_section;
pub mod cssstart_section;
pub mod divend_section;
pub mod divstart_section;
pub mod groups_section;
pub mod h1_section;
pub mod h2_section;
pub mod h3_section;
pub mod h4_section;
pub mod h5_section;
pub mod h6_section;
pub mod hr_section;
pub mod html_section;
pub mod htmlend_section;
pub mod htmlstart_section;
pub mod image_section;
pub mod list_section;
pub mod metadata_section;
pub mod nav_section;
pub mod note_section;
pub mod notes_section;
pub mod olist_section;
pub mod p_section;
pub mod pre_section;
pub mod preend_section;
pub mod prestart_section;
pub mod ref_section;
pub mod results_section;
pub mod resultsend_section;
pub mod resultsstart_section;
pub mod script_section;
pub mod scriptend_section;
pub mod scriptstart_section;
pub mod sectionend_section;
pub mod sectionstart_section;
pub mod subtitle_section;
pub mod title_section;
pub mod tldrend_section;
pub mod tldrstart_section;
pub mod todo_section;
pub mod vimeo_section;
pub mod warning_section;
pub mod warnings_section;
pub mod youtube_section;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NeoSection {
    ArticleEnd {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    ArticleStart {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
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
        list: Option<Vec<String>>,
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
    DivEnd {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    DivStart {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    Groups {
        list: Option<Vec<String>>,
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
    Comment {},
    Html {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<String>,
    },
    Hr {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
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
        prelude: Option<Vec<Block>>,
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
    Ref {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    Script {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<String>,
    },
    SectionEnd {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    SectionStart {
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
    TlDrEnd {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    TlDrStart {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    Todo {
        attributes: Option<Vec<AttributeV2>>,
        items: Option<Vec<Container>>,
        prelude: Option<Vec<Block>>,
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
            sectionstart_section,
            aside_section,
            articlestart_section,
            blockquote_section,
            checklist_section,
            html_section,
            image_section,
            list_section,
            olist_section,
            notes_section,
            note_section,
            codestart_section,
            cssstart_section,
            divstart_section,
            htmlstart_section,
            prestart_section,
            scriptstart_section,
            pre_section,
        )),
        alt((
            articleend_section,
            codeend_section,
            cssend_section,
            divend_section,
            htmlend_section,
            preend_section,
            scriptend_section,
            sectionend_section,
            groups_section,
            tldrstart_section,
            tldrend_section,
            vimeo_section,
            warning_section,
            warnings_section,
            youtube_section,
            todo_section,
            results_section,
            resultsend_section,
            resultsstart_section,
        )),
        alt((
            code_section,
            css_section,
            h1_section,
            h2_section,
            h3_section,
            h4_section,
            h5_section,
            h6_section,
            hr_section,
            ref_section,
            script_section,
            metadata_section,
            title_section,
            pre_section,
            subtitle_section,
            nav_section,
            categories_section,
            comment_section,
            bookmark_section,
            p_section,
        )),
    ))(source)?;
    Ok((source, section))
}
