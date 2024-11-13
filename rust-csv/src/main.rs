use ::std::error::Error;
use csv::Reader;


fn read_main_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(path)?;

    for i in reader.records() {
        let record = i?;
        println!("{:?}", record);
    }

    Ok(())
}
fn main() {
    if let Ok(()) = read_main_file("./revised.csv") {
         println!("File read successfully"); 
    } else {
    eprintln!("Some is wrong");
    }
}
