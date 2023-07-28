use crate::tokens::Token;
use nom::character::complete::line_ending;
use nom::combinator::not;
use nom::error::VerboseError;
use nom::sequence::pair;
use nom::IResult;

pub fn single_newline(source: &str) -> IResult<&str, Token, VerboseError<&str>> {
    let (source, _) = pair(line_ending, not(line_ending))(source)?;
    Ok((source, Token::Space))
}
