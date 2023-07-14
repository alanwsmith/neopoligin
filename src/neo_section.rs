use nom::IResult;
use nom::branch::alt;
use crate::sections::Section;
// use crate::blocks::Block;
// use crate::containers::Container;
// use crate::section_attrs::SecAttr;
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
use crate::sections::opensection::opensection;
use crate::sections::p::p;
use crate::sections::pre::pre;
use crate::sections::script::script;
use crate::sections::startcode::startcode;
use crate::sections::title::title;
use crate::sections::todo::todo;
use crate::sections::vimeo::vimeo;
use crate::sections::youtube::youtube;

pub fn neo_section(source: &str) -> IResult<&str, Section> {
    let (source, results) = alt((
        // order matters here so things don't get flipped
        alt((title, hidden, html, h)),
        alt((pre, p)),
        alt((code, list)),
        alt((notes, note, checklist, opensection)),
        alt((
            aside, blockquote, closediv, code, css, endcode, hr, image, opendiv, olist, pre,
            script, startcode, todo, vimeo, youtube,
        )),
    ))(source)?;
    Ok((source, results))
}