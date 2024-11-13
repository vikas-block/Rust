use select::document::Document;
use select::predicate::Name;
use error_chain::error_chain;

error_chain!{
    foreign_links{
       ReqError(reqwest::Error);
       IoError(std::io::Error);
    }
}

#[tokio::main]
async fn main()->Result<()> {
    
   let res= reqwest::get("https://en.wikipedia.org")
   .await?
   .text()
   .await?;

   let doc= Document::from(res.as_str());
   doc.find(Name("a"))
   .filter_map(|n| n.attr("href"))
   .for_each(|v| println!("{}",v));

   Ok(())
}
