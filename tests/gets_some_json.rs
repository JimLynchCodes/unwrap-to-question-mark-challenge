use assert_cmd::prelude::*;
use serde_json::{Result, Value};
use std::process::Command;
use std::str;

#[test]
fn gets_some_json() -> Result<()> {
    let output_bytes = Command::cargo_bin("unwrap_to_question_mark_challenge")
        .unwrap()
        .output()
        .unwrap()
        .stdout;

    let output_str = match str::from_utf8(&output_bytes) {
        Ok(val) => val,
        Err(_) => panic!("got non UTF-8 data from stdout"),
    };

    assert!(output_str.contains("activity"));

    Ok(())
}
