use error_chain::error_chain;
// use std::io::copy;
use std::fs::File;
use tempfile::Builder;
use std::io::copy;


error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()>  {

    //  let file_path = Builder::new().prefix("my_file");

    let url ="https://img.freepik.com/free-psd/rusty-3d-editable-text-effect_125540-7176.jpg?semt=ais_hybrid";
    
    let response = reqwest::get(url).await?;
    
    let mut dest ={
        let fname = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name | if name.is_empty() { None } else {Some(name)})
        .unwrap_or("tmp.bin");

      
    let fname = std::path::Path::new("/home/rishabhjaiswal/Downloads").join(fname);
      println!("file path is '{:?}'",fname);
      File::create(fname)?
    };


    let content =  response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
