#![allow(unused_imports)]
// use neopolengine::neo_sections::neo_sections;
// use neopolengine::neo_sections::Section;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestParts {
    input: String,
    expected: Vec<Section>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SectionTestCase {
    ignore: Option<bool>,
    solo: Option<bool>,
    name: String,
    notes: Vec<String>,
    parts: TestParts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestShell {
    spec_tests: Vec<SectionTestCase>,
    sections_new_version: Vec<SectionTestCase>,
}

#[test]
fn solo_test_specs() {
    let json_text = fs::read_to_string("./tests/neopolitan_spec_tests.json").unwrap();
    let test_data: TestShell = serde_json::from_str(json_text.as_str()).unwrap();
    test_data
        .sections_new_version
        .iter()
        .filter(|t| match t.solo {
            Some(just_me) if just_me == true => true,
            _ => false,
        })
        .into_iter()
        .for_each(|x| {
            dbg!(&x.parts.input);
            let results = neo_sections_new_version(&x.parts.input).unwrap().1;
            assert_eq!(x.parts.expected, results);
            ()
        });

    // These are the old tests that should be
    // removed once the sections_v2 stuff is onlin
    let section_test_cases: TestShell = serde_json::from_str(json_text.as_str()).unwrap();
    // do solo tests first
    section_test_cases
        .spec_tests
        .iter()
        .filter(|t| match t.solo {
            Some(just_me) if just_me == true => true,
            _ => false,
        })
        .into_iter()
        .for_each(|x| {
            dbg!(&x.parts.input);
            let results = neo_sections(&x.parts.input).unwrap().1;
            assert_eq!(x.parts.expected, results);
            ()
        });

    // then loop through everything that's not ignored
    // (this redoes the solo tests, but that's fine for now)
    section_test_cases
        .spec_tests
        .iter()
        .filter(|t| match t.ignore {
            Some(skip_me) if skip_me == true => false,
            _ => true,
        })
        .into_iter()
        .for_each(|x| {
            // dbg!(&x.parts.input);
            let results = neo_sections(&x.parts.input).unwrap().1;
            assert_eq!(x.parts.expected, results);
            ()
        });

    //
}
