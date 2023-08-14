#![allow(unused_imports)]
use pretty_assertions::{assert_eq, assert_ne};
use neopolengine::neo_sections::NeoSection;
use neopolengine::page::Page;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestParts {
    solo: Option<bool>,
    input: String,
    expected: Vec<NeoSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestShell {
    tests: Vec<TestParts>,
}

#[test]
fn sections_test() {
    let paths = fs::read_dir("./spec/sections").unwrap();
    for path in paths {
        let file_path = path.unwrap().path().display().to_string();
        if file_path.find(".json") != None {
            dbg!(&file_path);
            let json_text = fs::read_to_string(&file_path).unwrap();
            let test_shell: TestShell = serde_json::from_str(json_text.as_str()).unwrap();
            // Run Solo Tests First
            test_shell.tests.into_iter().for_each(|test| {
                if let Some(_) = test.solo {
                    let expected = test.clone().expected;
                    let mut p = Page::new_from(&test.input);
                    let results = p.raw_sections();
                    assert_eq!(expected, results);
                }
                // Then run everything
                let expected = test.expected;
                let mut p = Page::new_from(&test.input);
                let results = p.raw_sections();
                assert_eq!(expected, results);
            });
        }
    }
}

#[test]
fn tokens_test() {
    let paths = fs::read_dir("./spec/tokens").unwrap();
    for path in paths {
        let file_path = path.unwrap().path().display().to_string();
        if file_path.find(".json") != None {
            let json_text = fs::read_to_string(file_path).unwrap();
            let test_shell: TestShell = serde_json::from_str(json_text.as_str()).unwrap();
            // Run Solo Tests First
            test_shell.tests.into_iter().for_each(|test| {
                if let Some(_) = test.solo {
                    let expected = test.clone().expected;
                    let mut p = Page::new_from(&test.input);
                    let results = p.raw_sections();
                    assert_eq!(expected, results);
                }
                // Then run everything
                let expected = test.expected;
                let mut p = Page::new_from(&test.input);
                let results = p.raw_sections();
                assert_eq!(expected, results);
            });
        }
    }
}