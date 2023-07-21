#![allow(unused_imports)]
use neopolengine::page::page;
use neopolengine::page::Page;
use neopolengine::sections::sections;
use neopolengine::sections::Section;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestParts {
    input: String,
    expected: Page,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SectionTestCase {
    ignore: Option<bool>,
    solo: Option<bool>,
    name: String,
    notes: Option<Vec<String>>,
    parts: TestParts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestShell {
    attributes: Vec<SectionTestCase>,
    inline_tags: Vec<SectionTestCase>,
    pages: Vec<SectionTestCase>,
}

#[test]
fn solo_test_specs() {
    let json_text = fs::read_to_string("./spec.json").unwrap();
    let test_data: TestShell = serde_json::from_str(json_text.as_str()).unwrap();

    // This does all the sections via pages
    test_data.pages.iter().into_iter().for_each(|x| {
        dbg!(&x.parts.input);
        let results = page(&x.parts.input).unwrap().1;
        assert_eq!(x.parts.expected, results);
        ()
    });

    // // This does all the attributes
    // test_data.attributes.iter().into_iter().for_each(|x| {
    //     dbg!(&x.parts.input);
    //     let results = sections(&x.parts.input).unwrap().1;
    //     assert_eq!(x.parts.expected, results);
    //     ()
    // });

    // // This does all the inline_tags
    // test_data.inline_tags.iter().into_iter().for_each(|x| {
    //     dbg!(&x.parts.input);
    //     let results = sections(&x.parts.input).unwrap().1;
    //     assert_eq!(x.parts.expected, results);
    //     ()
    // });

    // // This runs any soloed tests
    // test_data
    //     .sections
    //     .iter()
    //     .filter(|t| match t.solo {
    //         Some(just_me) if just_me == true => true,
    //         _ => false,
    //     })
    //     .into_iter()
    //     .for_each(|x| {
    //         dbg!(&x.parts.input);
    //         let results = sections(&x.parts.input).unwrap().1;
    //         assert_eq!(x.parts.expected, results);
    //         ()
    //     });

    // // This skips anything that's not ignored
    // test_data
    //     .sections
    //     .iter()
    //     .filter(|t| match t.ignore {
    //         Some(skip_me) if skip_me == true => false,
    //         _ => true,
    //     })
    //     .into_iter()
    //     .for_each(|x| {
    //         dbg!(&x.parts.input);
    //         let results = sections(&x.parts.input).unwrap().1;
    //         assert_eq!(x.parts.expected, results);
    //         ()
    //     });

    // // These are the old tests that should be
    // // removed once the sections_v2 stuff is onlin
    // let section_test_cases: TestShell = serde_json::from_str(json_text.as_str()).unwrap();
    // // do solo tests first
    // section_test_cases
    //     .spec_tests
    //     .iter()
    //     .filter(|t| match t.solo {
    //         Some(just_me) if just_me == true => true,
    //         _ => false,
    //     })
    //     .into_iter()
    //     .for_each(|x| {
    //         dbg!(&x.parts.input);
    //         let results = neo_sections(&x.parts.input).unwrap().1;
    //         assert_eq!(x.parts.expected, results);
    //         ()
    //     });

    // // then loop through everything that's not ignored
    // // (this redoes the solo tests, but that's fine for now)
    // section_test_cases
    //     .spec_tests
    //     .iter()
    //     .filter(|t| match t.ignore {
    //         Some(skip_me) if skip_me == true => false,
    //         _ => true,
    //     })
    //     .into_iter()
    //     .for_each(|x| {
    //         // dbg!(&x.parts.input);
    //         let results = neo_sections(&x.parts.input).unwrap().1;
    //         assert_eq!(x.parts.expected, results);
    //         ()
    //     });

    //
}
