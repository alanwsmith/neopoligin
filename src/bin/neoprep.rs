#![allow(unused_imports)]
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::fs;
use std::fs::copy;
use std::path::PathBuf;

// This is currently hard coded to look for:
// `>> site: neoengine``

// [] Handle dir names if there is no id
// [] Handle explicit paths
// [] Configure site directory and subdirecotyr to load files independently
// [] Only allows speicifci nonce- words
// [] Only allows .neo extensions
// [] Only allows published and draft files
// [] Output sets to a `posts`` directory (done manually)
// [] Create `posts`` dir if it doesn't exist
// [] Each file has its own directory created for it
// [] The output file is always index.neo in the folder
// [] Makes any directories that need to exist
// [] Setup config file process

fn main() {
    println!("Running neoprep");

    let dev = (
        "/Users/alan/workshop/neopolengine/content_raw_samples",
        "/Users/alan/workshop/neopolengine/content",
    );
    let _prod = (
        "/Users/alan/Grimoire",
        "/Users/alan/workshop/neopolengine/content",
    );

    let dirs = dev.clone();

    let paths = filter_extensions(
        fs::read_dir(dirs.0)
            .unwrap()
            .into_iter()
            .map(|p| p.expect("here").path())
            .collect::<Vec<PathBuf>>(),
    );

    paths.iter().for_each(|p| {
        let data = fs::read_to_string(p).unwrap();

        match (
            filter_status(data.as_str()).unwrap().1,
            filter_site(data.as_str()).unwrap().1,
        ) {
            (true, true) => {
                let the_file_id = file_id(data.as_str()).unwrap().1.to_string();
                let output_dir_name = output_dir_name(
                    p.file_stem().unwrap().to_str().unwrap(),
                    the_file_id.as_str(),
                )
                .unwrap()
                .1;
                println!("Copying to: {}", &output_dir_name);
                let mut output_dir_path = PathBuf::from(dirs.1);
                output_dir_path.push(output_dir_name);
                match output_dir_path.try_exists() {
                    Ok(x) => {
                        if x == false {
                            fs::create_dir(&output_dir_path).unwrap();
                        }
                    }
                    Err(_) => {}
                }
                let mut output_file_path = output_dir_path.clone();
                output_file_path.push("index.neo");
                let _ = copy(p, output_file_path);
            }
            _ => {
                dbg!("skipping");
            }
        }

        // if let Ok((_, send_it)) = filter_status(data.as_str()) {
        //     dbg!("here");

        // if send_it {
        //     let file_id = file_id(data.as_str()).unwrap().1.to_string();
        //     let output_dir_name =
        //         output_dir_name(p.file_stem().unwrap().to_str().unwrap(), file_id.as_str())
        //             .unwrap()
        //             .1;
        //     println!("Copying to: {}", &output_dir_name);
        //     let mut output_dir_path = PathBuf::from(dirs.1);
        //     output_dir_path.push(output_dir_name);
        //     match output_dir_path.try_exists() {
        //         Ok(x) => {
        //             if x == false {
        //                 fs::create_dir(&output_dir_path).unwrap();
        //             }
        //         }
        //         Err(_) => {}
        //     }
        //     let mut output_file_path = output_dir_path.clone();
        //     output_file_path.push("index.neo");
        //     let _ = copy(p, output_file_path);
        // }
        // }

        ()
    });
}

pub fn file_id(source: &str) -> IResult<&str, &str> {
    let (a, _b) = take_until("\n-> attributes")(source)?;
    let (a, _b) = tag("\n-> attributes")(a)?;
    let (a, _b) = take_until(">> id: ")(a)?;
    let (a, _b) = tag(">> id: ")(a)?;
    let (a, _b) = multispace0(a)?;
    let (_a, b) = not_line_ending(a)?;
    Ok(("", b.trim()))
}

pub fn output_dir_name<'a>(source: &'a str, id: &'a str) -> IResult<&'a str, String> {
    let (source, _) = multispace0(source.trim())?;
    let (source, parts) = many_till(terminated(is_not(" -"), alt((is_a(" -"), eof))), eof)(source)?;
    let response = format!(
        "{}--{}",
        parts
            .0
            .iter()
            .map(|p| p.to_lowercase())
            .collect::<Vec<String>>()
            .join("-"),
        id
    );
    Ok((source, response))
}

pub fn filter_status(source: &str) -> IResult<&str, bool> {
    let (a, _b) = take_until("\n-> attributes")(source)?;
    let (a, _b) = tag("\n-> attributes")(a)?;
    let (a, _b) = take_until(">> status: ")(a)?;
    let (a, _b) = tag(">> status: ")(a)?;
    let (_a, b) = not_line_ending(a)?;
    match b.trim() {
        "published" => Ok(("", true)),
        "draft" => Ok(("", true)),
        _ => Ok(("", false)),
    }
}

pub fn filter_site(source: &str) -> IResult<&str, bool> {
    let (a, _b) = take_until("\n-> attributes")(source)?;
    let (a, _b) = tag("\n-> attributes")(a)?;
    let (a, _b) = take_until(">> site: ")(a)?;
    let (a, _b) = tag(">> site: ")(a)?;
    let (_a, b) = not_line_ending(a)?;
    match b.trim() {
        "neoengine" => Ok(("", true)),
        _ => Ok(("", false)),
    }
}

pub fn filter_extensions(list: Vec<PathBuf>) -> Vec<PathBuf> {
    list.into_iter()
        .filter(|p| match p.extension() {
            Some(ext) => {
                if ext == "neo" {
                    true
                } else {
                    false
                }
            }
            None => false,
        })
        .collect()
}

#[cfg(test)]

mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    pub fn filter_extensions_test() {
        let files = vec![
            PathBuf::from("/a/b/alfa.neo"),
            PathBuf::from("/a/b/bravo.txt"),
        ];
        assert_eq!(
            filter_extensions(files),
            vec![PathBuf::from("/a/b/alfa.neo")]
        );
    }

    #[test]
    pub fn filter_status_false() {
        // allowed statuses are hard coded above
        let lines = [
            "",
            "-> attributes",
            ">> id: 12341234",
            ">> status: unpublished",
        ];
        assert_eq!(filter_status(lines.join("\n").as_str()).unwrap().1, false);
    }

    #[test]
    pub fn filter_status_true() {
        let lines = [
            "",
            "-> attributes",
            ">> id: 12341234",
            ">> status: published",
        ];
        assert_eq!(filter_status(lines.join("\n").as_str()).unwrap().1, true);
    }

    #[test]
    pub fn filter_status_with_trailing_space() {
        let lines = [
            "",
            "-> attributes",
            ">> id: 12341234",
            ">> status: published ",
        ];
        assert_eq!(filter_status(lines.join("\n").as_str()).unwrap().1, true);
    }

    #[test]
    pub fn filter_site_test() {
        let lines = [
            "",
            "-> attributes",
            ">> id: 12341234",
            ">> site: neoengine",
            ">> status: published ",
        ];
        assert_eq!(filter_site(lines.join("\n").as_str()).unwrap().1, true);
    }

    // TODO Add test for not having a site attribute maybe?

    #[test]
    pub fn basic_output_dir_name() {
        let source = PathBuf::from("/some/posts/rust- Basic Path Example.neo");
        let id = String::from("1234qwer");
        let expected = Ok(("", "rust-basic-path-example--1234qwer".to_string()));
        let results = output_dir_name(source.file_stem().unwrap().to_str().unwrap(), id.as_str());
        assert_eq!(results, expected);
    }

    #[test]
    #[ignore]
    pub fn dir_with_other_stuff() {
        let source = PathBuf::from("/some/posts/alfa-  Bravo -- Charlie.neo");
        let id = String::from("9876rewq");
        let expected = Ok(("", "bravo-charlie--9876rewq".to_string()));
        let results = output_dir_name(source.file_stem().unwrap().to_str().unwrap(), id.as_str());
        assert_eq!(results, expected);
    }

    #[test]
    #[ignore]
    pub fn dir_with_dashes_that_are_not_followed_by_a_space() {
        let source = PathBuf::from("alfa-bravo");
        let id = String::from("9876rewq");
        let expected = Ok(("", "bravo-charlie--9876rewq".to_string()));
        let results = output_dir_name(source.file_stem().unwrap().to_str().unwrap(), id.as_str());
        assert_eq!(results, expected);
    }

    #[test]
    pub fn file_id_basic() {
        let lines = ["", "-> attributes", ">> id: 1234alfa"].join("\n");
        assert_eq!(file_id(lines.as_str()).unwrap().1, "1234alfa");
    }

    #[test]
    pub fn file_id_with_trailing_white_space() {
        let lines = [
            "",
            "-> attributes",
            ">> id: 6789bravo ",
            ">> status: published",
            "",
        ]
        .join("\n");
        assert_eq!(file_id(lines.as_str()).unwrap().1, "6789bravo");
    }
}
