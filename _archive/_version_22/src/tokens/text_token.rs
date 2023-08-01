use crate::tokens::Token;
use nom::bytes::complete::is_not;
use nom::error::VerboseError;
use nom::IResult;

pub fn text_token(source: &str) -> IResult<&str, Token, VerboseError<&str>> {
    let (source, string) = is_not("< \n\t\r")(source)?;
    Ok((
        source,
        Token::Text {
            string: string.to_string(),
        },
    ))
}
