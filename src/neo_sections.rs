#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(dead_code)]
use crate::attributes::attributes;
use crate::attributes::AttributesObj;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
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
use nom::bytes::complete::tag_no_case;
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


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum NeoSection {
    Aside {
        attributes: Option<AttributesObj>,
        content: Option<Vec<Block>>,
    },
    List {
        attributes: Option<AttributesObj>,
        items: Option<Vec<Container>>,
        preface: Option<Vec<Block>>,
    },
    Title {
        attributes: Option<AttributesObj>,
        content: Option<Vec<Block>>,
        headline: Option<Block>,
    },
    P {
        attributes: Option<AttributesObj>,
        content: Option<Vec<Block>>,
    },
    RawPageAttributes(Vec<(String, String)>),
}