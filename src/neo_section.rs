use nom::IResult;
use nom::branch::alt;
use crate::neo_sections::Section;
// use crate::blocks::Block;
// use crate::containers::Container;
// use crate::section_attrs::SecAttr;
use crate::neo_sections::aside::aside;
use crate::neo_sections::blockquote::blockquote;
use crate::neo_sections::checklist::checklist;
use crate::neo_sections::closediv::closediv;
use crate::neo_sections::endsection::endsection;
use crate::neo_sections::code::code;
use crate::neo_sections::css::css;
use crate::neo_sections::endcode::endcode;
use crate::neo_sections::h::h;
use crate::neo_sections::hidden::hidden;
use crate::neo_sections::hr::hr;
use crate::neo_sections::html::html;
use crate::neo_sections::image::image;
use crate::neo_sections::list::list;
use crate::neo_sections::note::note;
use crate::neo_sections::notes::notes;
use crate::neo_sections::olist::olist;
use crate::neo_sections::opendiv::opendiv;
use crate::neo_sections::section::section;
use crate::neo_sections::p::p;
use crate::neo_sections::pre::pre;
use crate::neo_sections::script::script;
use crate::neo_sections::startcode::startcode;
use crate::neo_sections::title::title;
use crate::neo_sections::todo::todo;
use crate::neo_sections::vimeo::vimeo;
use crate::neo_sections::youtube::youtube;

pub fn neo_section(source: &str) -> IResult<&str, Section> {
    let (source, results) = alt((
        // order matters here so things don't get flipped
        alt((title, hidden, html, h)),
        alt((pre, p)),
        alt((code, list)),
        alt((notes, note, checklist, section, endsection)),
        alt((
            aside, blockquote, closediv, code, css, endcode, hr, image, opendiv, olist, pre,
            script, startcode, todo, vimeo, youtube,
        )),
    ))(source.trim_start())?;
    Ok((source, results))
}