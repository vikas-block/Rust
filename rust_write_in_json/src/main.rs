use::serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
struct  Paragraph{
    name:String,


}

#[derive(Serialize,Deserialize)]
struct Article{
    title:String,
    owner:String,
    paragraph:Vec<Paragraph>
}
fn main() {
    

   let article:Article=Article{
    title:String::from("Web3.0"),
    owner:String::from("Internet"),
    paragraph:vec![
      Paragraph{
            name: String::from("Blockchain")
        }, 
        Paragraph{
            name: String::from("artificial Intelligence")
        }
    ]
   };

   let data = serde_json::to_string(&article).unwrap();
   println!("Json data :- {}", data);

}
