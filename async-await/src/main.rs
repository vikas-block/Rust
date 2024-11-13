use error_chain::error_chain;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main()->Result<()> {

    let res = reqwest::get("http:httpbin.org/get").await?;
    println!("status :\n{:?}",res.status());
    println!("header :\n{:#?}",res.headers());
    let body = res.text().await?;
    println!("Body : \n{}",body);
    println!("Hello, world!");
    Ok(())
}
