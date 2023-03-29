//!    This code technically works, but the function signature tells us nothing about what kinds of errors it can throw.
//!
//!    Also, the unwrap will crash all threads of our program.
//!
//!    The goal here is to refactor this code to change all "unwrap()" calls to ? operator...
//!
//!    BONUS - Try to change "unwrap" to ? in the integration test and in main.rs as well!

use reqwest::blocking::get;
use serde_json::Value;
use std::error::Error;

pub fn get_some_json() -> Result<Value, Box::<dyn Error>> {
    const URL: &str = "https://www.boredapi.com/api/activity";
    
    let resp = get(URL)?.text()?;
    
    serde_json::from_str::<Value>(&resp).map_err(Box::<dyn Error>::from)
}
