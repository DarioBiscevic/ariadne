#![allow(unused)] //Temporary

use clap::Parser;

mod prelude;
mod error;

use crate::prelude::*;

fn main() -> Result<()>{
    let args = Args::parse();

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    ///Input image filename
    #[arg(short, long)]
    filename: String
}