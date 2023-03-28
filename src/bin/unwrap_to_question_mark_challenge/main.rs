mod api_caller;
use api_caller::get_some_json;

fn main() {    
    let dynamic_json = get_some_json().expect("Calling for json failed. 😔");
    println!("some json: {:#?}", dynamic_json);
}
