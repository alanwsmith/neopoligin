use crate::attributes::attributes;
use crate::attributes::AttributesObj;
use crate::blocks::list_paragraph_block::list_paragraph_block;
use crate::containers::Container;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::opt;
use nom::error::VerboseError;
use nom::error::VerboseErrorKind;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub fn list_item_container(source: &str) -> IResult<&str, Container, VerboseError<&str>> {
    let (source, _) = tag("- ")(source)?;
    let (source, content) = many1(preceded(multispace0, list_paragraph_block))(source)?;
    Ok((source, Container::ListItem { content }))
}
