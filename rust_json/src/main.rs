use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    title: String,
    owner: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
    {
    "title":"Web3.0",
    "owner":"internet",
    "paragraph":[
        {
        "name":"Blockchain"
        },
        {
        "name":"Artificial Intelligenace"
        }
    ]
   }"#;

   let json_data:Article=read_json_file(&json);
   println!("Name: {:#?}", json_data.title);
}

fn read_json_file(json:&str)->Article{

    let data:Article=serde_json::from_str(&json).unwrap();
    return data;
}
