use crate::tokens::Token;
use nom::character::complete::space1;
use nom::error::VerboseError;
use nom::IResult;

pub fn space_token(source: &str) -> IResult<&str, Token, VerboseError<&str>> {
    let (source, _) = space1(source)?;
    Ok((source, Token::Space))
}
