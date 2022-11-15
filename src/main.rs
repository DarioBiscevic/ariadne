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
    match Reader::open(args.filename){
        Ok(file) => {
            //Check if the file is an image
            match file.decode(){
                Ok(image) => run(image.into_rgb8(), args.algorithm),
                Err(error) => Err(Error::IOError(error.to_string()))
            }
        },

        Err(error) => Err(Error::IOError(error.to_string()))
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    ///Input image filename
    #[arg(short, long, required = true)]
    filename: String,

    ///Pathfinding algorithm
    #[arg(value_enum, short, long, default_value_t = Algorithm::Dijkstra)]
    algorithm: Algorithm,
}