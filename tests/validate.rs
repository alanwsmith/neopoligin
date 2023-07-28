#![allow(unused_imports)]
use neopolengine::neo_sections::NeoSection;
use neopolengine::page::Page;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestParts {
    input: String,
    expected: Vec<NeoSection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestShell {
    tests: Vec<TestParts>,
}

#[test]
fn solo_test_specs() {
    let paths = fs::read_dir("./tests/spec/sections").unwrap();
    for path in paths {
        let file_path = path.unwrap().path().display().to_string();
        if file_path.find(".json") != None {
            let json_text = fs::read_to_string(file_path).unwrap();
            let test_shell: TestShell = serde_json::from_str(json_text.as_str()).unwrap();
            test_shell.tests.into_iter().for_each(|test| {
                let mut p = Page::new_from(&test.input);
                assert_eq!(test.expected, p.raw_sections());
            });
        }
    }
}
