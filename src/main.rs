use clap::Parser;
use image::io::Reader;

mod prelude;
mod utils;
mod error;

use crate::prelude::*;

fn main() -> Result<()>{
    //Parse the arguments
    let args = Args::parse();

    //Check if the specified file exists
    match Reader::open(&args.filename){
        Ok(file) => {
            //Check if the file is an image
            match file.decode(){
                Ok(image) => run(image.into_rgb8(), args),
                Err(error) => Err(Error::IOError(error.to_string()))
            }
        },

        Err(error) => Err(Error::IOError(error.to_string()))
    }
}