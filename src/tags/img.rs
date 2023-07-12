#![allow(unused_imports)]
use crate::tag_attrs::tag_attrs;
use crate::tags::Tag;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::combinator::opt;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::sequence::terminated;
use nom::branch::alt;
use nom::IResult;
use nom::Parser;
use nom::combinator::peek;
use crate::tag_attrs::id::id;
use nom::multi::many0;
use crate::tags::TagAttr;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn img(source: &str) -> IResult<&str, Tag> {
    let (source, src) = delimited(
        tag("<<"),
        is_not("|").map(|s: &str| s.to_string()),
        tuple((tag("|"), tag_no_case("img"))),
    )(source)?;
    // dbg!(source);
    let (source, alt_text) = opt(
        preceded(
            tag("|"),
            terminated(
                is_not("|:>").map(|s: &str| s.to_string()),
                alt((
                    peek(tag("|")), peek(tag(">"))
                ))
            )
        )
    )(source)?;

    let (source, mut attrs) = tag_attrs(source)?;
    let (source, _) = tag(">>")(source)?;

    let _found_it = attrs.iter_mut().find(|x| match x {
        TagAttr::Class(_) => {true},
        _ => { false }
    });


    let n1 = get_path_to_file(&src);
    if let Some(path_part) = n1 {
        let n3 = path_part.to_str();
        let n4 = n3.unwrap();
        let n5 = n4.to_string();
        Ok((source, Tag::Img{ attrs, src: n5, alt_text, caption: None}))
    } else {
        Ok((source, Tag::Img{ attrs, src, alt_text, caption: None}))
    }
    // let n2 = n1.unwrap();
    // let n3 = n2.to_str();
    // let n4 = n3.unwrap();
    // let n5 = n4.to_string();



    // match (found_it, alt_text) {
    //     (Some(TagAttr::Class(the_thing)), Some(the_lang)) => {
    //         the_thing.push(format!("language-{}", the_lang));
    //     }
    //     (None, Some(the_lang)) => {
    //         attrs.push(TagAttr::Class(vec![format!("language-{}", the_lang)]))
    //     },
    //     _ => {}
    // }

     // Ok((source, Tag::Img{ attrs, src, alt_text, caption: None}))
}


fn get_path_to_file(target_name: &str) -> Option<PathBuf> {
    // dbg!("----------------------");
    // dbg!(&target_name);
    // dbg!("----------------------");
    let site_root = "/Users/alan/workshop/alanwsmith.com/content";
    let files: Vec<_> = WalkDir::new(site_root)
        .into_iter()
        .filter_map(|v| {
            if let Some(name) = v.as_ref().unwrap().path().file_stem() {
                if name == target_name {
                    dbg!(&target_name);
                    let dir = v.as_ref().unwrap().path().strip_prefix(site_root);
                    let mut return_path_buf = PathBuf::from("/");
                    return_path_buf.push(dir.unwrap().to_str().unwrap());
                    //dbg!(&return_path_buf);
                    Some(return_path_buf)
                    // v.ok()
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    if files.len() == 1 {
        Some(files[0].clone())
        // let new_path = PathBuf::from(files[0].unwrap());
        // let new_path = files[0].to_path_buf().strip_prefix(site_root).unwrap();
        // dbg!(&new_path);
        // let new_path2 = &new_path.to_str().unwrap();
        // Some("".to_string())
    } else {
        None
    }
}




#[cfg(test)]
mod test {
    use super::*;
    use crate::tag_attrs::TagAttr;
    use rstest::rstest;

    #[rstest]
    #[case(
        "<<some-image-name|img>>",
        Tag::Img{ attrs: vec![], alt_text: None, 
            src: "some-image-name".to_string(),
            caption: None,
        }
    )]
    #[case(
        "<<some-image|img|alt text here>>",
        Tag::Img{ 
            attrs: vec![],
            alt_text: Some("alt text here".to_string()),
            src: "some-image".to_string(),
            caption: None, 
            },
    )]
    #[case(
        "<<the-path|img|some alt text|id: bravo|class: echo>>",
        Tag::Img{ attrs: vec![
            TagAttr::Id("bravo".to_string()),
            TagAttr::Class(vec!["echo".to_string()])
        ],  alt_text: Some("some alt text".to_string()) ,
            src: "the-path".to_string(),
            caption: None, 
        }
    )]
    #[case(
        "<<delta-path|img|bravo text|id: delta>>",
        Tag::Img { attrs: vec![
            TagAttr::Id("delta".to_string()),
        ],  alt_text: Some("bravo text".to_string()) ,
            caption: None, 
            src: "delta-path".to_string()
        }
    )]
    #[case(
        "<<alfa-path|img|class: tango>>",
        Tag::Img{ 
            attrs: vec![
                TagAttr::Class(vec!["tango".to_string()])
            ], 
            alt_text: None, 
            caption: None, 
            src: "alfa-path".to_string(), 
        }
    )]
    fn solo_link_test(#[case] i: &str, #[case] e: Tag) {
        assert_eq!(e, img(i).unwrap().1);
    }

// 
}
