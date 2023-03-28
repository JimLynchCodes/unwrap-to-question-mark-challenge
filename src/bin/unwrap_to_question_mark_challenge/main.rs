mod api_caller;
use api_caller::get_some_json;

fn main() {    
    let dynamic_json = get_some_json().unwrap();
    println!("some json: {:#?}", dynamic_json);
}
