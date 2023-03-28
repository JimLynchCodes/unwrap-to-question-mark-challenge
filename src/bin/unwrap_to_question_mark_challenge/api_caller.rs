//!    This code technically works, but the function signature tells us nothing about what kinds of errors it can throw.
//!
//!    Also, the unwrap will crash all threads of our program.
//!
//!    The goal here is to refactor this code to change all "unwrap()" calls to ? operator...
//!
//!    BONUS - Try to change "unwrap" to ? in the integration test and in main.rs as well!

use reqwest::blocking::get;
use serde_json::{Result, Value};

pub fn get_some_json() -> Result<Value> {
    const url: &str = "https://www.boredapi.com/api/activity";
    let resp = get(url).unwrap().text().unwrap();
    let json: Value = serde_json::from_str(&resp).unwrap();
    Ok(json)
}
