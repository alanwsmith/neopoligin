use crate::tokens::Token;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::IResult;

pub fn link_token(source: &str) -> IResult<&str, Token, VerboseError<&str>> {
    let (source, _) = tag("<<link|")(source)?;
    let (source, string) = is_not("|")(source)?;
    let (source, _) = tag("|")(source)?;
    let (source, url) = is_not(">")(source)?;
    let (source, _) = tag(">>")(source)?;
    Ok((
        source,
        Token::Link {
            string: string.to_string(),
            url: url.to_string(),
        },
    ))
}
