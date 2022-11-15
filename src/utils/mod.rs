use image::DynamicImage;

pub mod algorithm;
mod node;

use crate::prelude::*;
use node::Node;

pub use crate::utils::algorithm::Algorithm;

///Entry point for the main process. It computes the path from the starting point to the ending point
/// using the specified `algorithm`.
pub fn run(_image: DynamicImage, _algorithm: Algorithm) -> Result<()>{
    //TODO: generate node tree, find starting and ending point, execute algorithm, create output file

    Ok(())
}