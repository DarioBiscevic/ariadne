use crate::prelude::*;
use image::DynamicImage;

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Algorithm{
    Dijkstra
}

///Entry point for the main process. It computes the path from the starting point to the ending point
/// using the specified `algorithm`.
pub fn run(_image: DynamicImage, _algorithm: Algorithm) -> Result<()>{


    Ok(())
}