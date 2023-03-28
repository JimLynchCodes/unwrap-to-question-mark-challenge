use reqwest::blocking::{get};
// use reqwest::header::{HeaderValue, COOKIE, USER_AGENT, HeaderMap};
use serde_json::{Result, Value};
// use serde_json::{Value};

pub fn get_some_json() -> Result<Value> {
    const url: &str = "https://www.boredapi.com/api/activity";

    let resp: String = get(url).unwrap().text().unwrap();
    let json: Value = serde_json::from_str(&resp).unwrap();

    

    // println!("name: {}", json["name"]);
    // println!("age: {}", json["age"]);
    // println!("age: {}", json["price"]);

    // let price: f64 = json["price"] as f64;

    let price: f64 = json["price"].as_f64().unwrap();
    let accessibility: f64 = json["accessibility"].as_f64().unwrap();

    println!("{:#?}", price + accessibility);

    Ok(json)
}