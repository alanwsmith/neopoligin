use crate::attributes::*;
use crate::tokens::Token;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::IResult;
use nom::character::complete::space0;
use nom::combinator::opt;
use nom::multi::many1;

pub fn link_token(source: &str) -> IResult<&str, Token, VerboseError<&str>> {
    // <<link|a|path.html|class: bravo>>
    // dbg!(&source);
    let (source, _) = tag("<<link")(source)?;
    // dbg!(&source);
    let (source, _) = space0(source)?;
    // dbg!(&source);
    let (source, _) = tag("|")(source)?;
    // dbg!(&source);
    let (source, string) = is_not("|")(source)?;
    // dbg!(&source);
    let (source, _) = tag("|")(source)?;
    // dbg!(&source);
    let (source, url) = is_not("|>")(source)?;
    // dbg!(&source);
    let (source, attributes) = opt(many1(attribute))(source)?;
    // dbg!(&source);
    // dbg!(&source);
    let (source, _) = tag(">>")(source)?;
    // dbg!(&source);
    Ok((
        source,
        Token::Link {
            attributes,
            string: string.to_string(),
            url: url.to_string(),
        },
    ))
}
