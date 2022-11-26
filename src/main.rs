use clap::Parser;
use image::io::Reader;

use std::time::Instant;

mod prelude;
mod utils;
mod error;

use crate::prelude::*;

fn main() -> Result<()>{
    //Parse the arguments
    let args = Args::parse();

    let start = Instant::now();

    //Check if the specified file exists
    match Reader::open(&args.filename){
        Ok(file) => {
            //Check if the file is an image
            match file.decode(){
                Ok(image) => {
                    let result = run(image.into_rgb8(), args.clone());
                    if args.logging{
                        println!("Total execution time: {:?}", start.elapsed());
                    }
                    result
                },
                Err(error) => Err(Error::IOError(error.to_string()))
            }
        },

        Err(error) => Err(Error::IOError(error.to_string()))
    }
}