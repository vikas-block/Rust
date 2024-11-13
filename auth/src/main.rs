use reqwest::blocking::Client;
use reqwest::Error;

fn main(){

    let user ="vikas".to_string();
    let passwd  = "NULL".to_string();
    let client = Client::new();
    let response = client
    .get("http://httpbin.org/get")
    .basic_auth(user, Some(passwd))
    .send();
println!("response: {:?}", response);
}