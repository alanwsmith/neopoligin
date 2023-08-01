// use crate::helpers::spacer_line::spacer_line;
use crate::tokens::link_token::link_token;
// use crate::tokens::single_newline::single_newline;
use crate::tokens::space_token::space_token;
use crate::tokens::text_token::text_token;
use nom::branch::alt;
// use nom::bytes::complete::is_not;
// use nom::combinator::not;
use nom::error::VerboseError;
use nom::IResult;
use serde::{Deserialize, Serialize};

pub mod link_token;
pub mod single_newline;
pub mod space_token;
pub mod text_token;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Token {
    Link { url: String, string: String },
    Space,
    Text { string: String },
}

pub fn token(source: &str) -> IResult<&str, Token, VerboseError<&str>> {
    // dbg!(&source);
    // let (source, _) = is_not("\n")(source)?;
    // dbg!(&source);
    let (source, token) = alt((text_token, link_token, space_token))(source)?;
    // dbg!(&token);
    // dbg!(&source);
    Ok((source, token))
}
