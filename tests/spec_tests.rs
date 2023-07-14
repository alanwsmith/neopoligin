use rstest::rstest;

use serde_json::Result;
use serde_json::Value;

#[rstest]
#[case("a", "a")]
fn run_test(#[case] input: &str, #[case] expected: &str) {
    let results = input;
    assert_eq!(expected, input);
}
