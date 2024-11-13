use error_chain::error_chain;
use std::io::Read;

error_chain! {


     foreign_links{
       Io(std::io::Error);
       HttpRequest(reqwest::Error);
     }


}

fn main(){
    let mut res = reqwest::blocking::get("http:httpbin.org/get").unwrap();

    let mut body = String::new();

    res.read_to_string(&mut body).unwrap();

    println!("status : {}", res.status());
    println!("header : \n{:#?}", res.headers());
    println!("body : \n{}", body);



  
}
