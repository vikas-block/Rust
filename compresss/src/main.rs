extern crate flate2;

use flate2::Compression;
use flate2::write::GzEncoder; 
use std::fs::File;
use std::env::args;
use std::io::BufReader;
use std::io::copy;

use std::time::Instant;



fn main() {
   if args().len()!=3{
    eprintln!("Usage: `source` `target`");
    return;
   }

   let mut input =  BufReader::new(File::open(args().nth(1).unwrap()).unwrap());
   let output = File::create(args().nth(2).unwrap()).unwrap();
   let mut encoder = GzEncoder::new(output, Compression::fast());
   let start= Instant::now();
   copy(&mut input, &mut encoder).unwrap();

   let output = encoder.finish().unwrap();
   println!("source size : {:?}", input.get_ref().metadata().unwrap().len());
   println!("Target size : {:?}",output.metadata().unwrap().len());
   println!("time : {:?}", start.elapsed());



}

