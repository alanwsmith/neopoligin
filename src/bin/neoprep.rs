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
use nom::combinator::opt;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::fs;
use std::fs::copy;
use std::path::PathBuf;

// This is currently hard coded to look for:
// `>> site: neoengine``

// [] Add a note to the top of the files saying that they are copies
//  to help prevenet editing the wrong ones
// [] Setup dir paths if there is no id
// [] Handle explicit paths
// [] Configure site directory and subdirecotyr to load files independently
// [] Only allows speicifci nonce- words
// [x] Only allows .neo extensions
// [x] Only allows published and draft files
// [x] Make direcotires if necessary
// [x] Ouput goes into a sub directory if one is defined
// [x] Each file has its own directory created for it
// [x] The output file is always index.neo in the folder
// [] Move config to file
// [] Add test to confirm files don't move if no site is found

#[derive(Clone)]
struct Config {
    output_root: String,
    output_sub_dir: Option<String>,
    input_root: String,
}

fn main() {
    println!("Running neoprep");

    let dev = Config {
        input_root: "/Users/alan/workshop/neopolengine/content_raw_samples".to_string(),
        output_root: "/Users/alan/workshop/neopolengine/content".to_string(),
        output_sub_dir: Some("pages".to_string()),
    };

    let config = dev.clone();

    let paths = filter_extensions(
        fs::read_dir(&config.input_root)
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
                let output_dir_path = PathBuf::from(&config.output_root);

                let mut output_file_path = output_dir_path.clone();
                match override_path(data.as_str()).unwrap().1 {
                    Some(path) => {
                        output_file_path.push(path);
                    }
                    None => {
                        match &config.output_sub_dir {
                            Some(dir) => {
                                output_file_path.push(dir);
                            }
                            None => {}
                        }
                        output_file_path.push(
                            output_dir_name(
                                p.file_stem().unwrap().to_str().unwrap(),
                                the_file_id.as_str(),
                            )
                            .unwrap()
                            .1,
                        );
                        output_file_path.push("index.neo");
                    }
                }

                match output_file_path.parent() {
                    Some(path) => match path.try_exists() {
                        Ok(check) => {
                            dbg!(&path);
                            if check == false {
                                fs::create_dir_all(path).unwrap();
                            }
                        }
                        Err(_) => {}
                    },
                    None => {}
                }

                // match output_dir_path.try_exists() {
                //     Ok(x) => {
                //         if x == false {
                //             fs::create_dir_all(&output_dir_path).unwrap();
                //         }
                //     }
                //     Err(_) => {}
                // }

                println!("Copying to: {:?}", &output_file_path);
                let _ = copy(p, output_file_path);
            }
            _ => {
                dbg!("skipping");
            }
        }
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

pub fn override_path(source: &str) -> IResult<&str, Option<String>> {
    let (source, _) = pair(take_until("\n-> attributes"), tag("\n-> attributes"))(source)?;
    let (source, the_path) = opt(preceded(
        pair(take_until(">> path: "), tag(">> path: ")),
        not_line_ending.map(|s: &str| s.to_string()),
    ))(source)?;
    Ok((source, the_path))
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

    #[test]
    pub fn basic_output_dir_name() {
        let source = PathBuf::from("/some/posts/rust- Basic Path Example.neo");
        let id = String::from("1234qwer");
        let expected = Ok(("", "rust-basic-path-example--1234qwer".to_string()));
        let results = output_dir_name(source.file_stem().unwrap().to_str().unwrap(), id.as_str());
        assert_eq!(results, expected);
    }

    #[test]
    pub fn dir_with_dashes_that_are_not_followed_by_a_space() {
        let source = PathBuf::from("alfa-bravo");
        let id = String::from("9876rewq");
        let expected = Ok(("", "alfa-bravo--9876rewq".to_string()));
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

    #[test]
    pub fn get_override_path() {
        let lines = ["", "-> attributes", ">> path: index.neo", ""].join("\n");
        assert_eq!(
            override_path(lines.as_str()).unwrap().1,
            Some("index.neo".to_string())
        );
    }
}
