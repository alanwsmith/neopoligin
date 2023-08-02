use crate::attributes::*;
use crate::blocks::Block;
use crate::containers::Container;
use crate::neo_sections::aside_section::aside_section;
use crate::neo_sections::blockquote_section::blockquote_section;
use crate::neo_sections::checklist_section::checklist_section;
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
use crate::neo_sections::image_section::image_section;
use crate::neo_sections::list_section::list_section;
use crate::neo_sections::note_section::note_section;
// use crate::neo_sections::metadata_section::metadata_section;
use crate::neo_sections::p_section::p_section;
use crate::neo_sections::startcode_section::startcode_section;
use crate::neo_sections::startdiv_section::startdiv_section;
use crate::neo_sections::startresults_section::startresults_section;
use crate::neo_sections::startsection_section::startsection_section;
use crate::neo_sections::title_section::title_section;
use crate::neo_sections::warning_section::warning_section;
use crate::neo_sections::warnings_section::warnings_section;
use crate::neo_sections::youtube_section::youtube_section;
use nom::error::VerboseError;
use nom::IResult;
use nom::{branch::alt, character::complete::multispace0};
use serde::{Deserialize, Serialize};

pub mod aside_section;
pub mod blockquote_section;
pub mod checklist_section;
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
pub mod image_section;
pub mod list_section;
pub mod note_section;

// pub mod metadata_section;

pub mod p_section;
pub mod startcode_section;
pub mod startdiv_section;
pub mod startresults_section;
pub mod startsection_section;
pub mod title_section;
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
    Checklist {
        attributes: Option<Vec<AttributeV2>>,
        items: Option<Vec<Container>>,
        preface: Option<Vec<Block>>,
    },
    Code {
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
    Note {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    // MetaData {
    //     attributes: Option<Vec<AttributeV2>>,
    // },
    P {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
    },
    Results {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<String>,
    },
    StartDiv {
        attributes: Option<Vec<AttributeV2>>,
    }, 
    StartSection {
        attributes: Option<Vec<AttributeV2>>,
    }, 
    Title {
        attributes: Option<Vec<AttributeV2>>,
        body: Option<Vec<Block>>,
        headline: Option<Block>,
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
            aside_section,
            blockquote_section,
            checklist_section,
        image_section,
        h1_section,
        h2_section,
        h3_section,
        h4_section,
        h5_section,
        h6_section,
        list_section,
        note_section,
        //     metadata_section,
        p_section,
        startcode_section,
        startdiv_section,
        startresults_section,
        startsection_section,
        title_section,
        warning_section,
        warnings_section,
        youtube_section,
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
        endtldr_section,
    ))
    ))(source)?;
    Ok((source, section))
}
