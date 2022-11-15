use image::{Rgb, RgbImage};

pub mod algorithm;
mod node;

use crate::prelude::*;
use node::Node;

pub use crate::utils::algorithm::Algorithm;

///Entry point for the main process. It computes the path from the starting point to the ending point
/// using the specified `algorithm`.
pub fn run(image: RgbImage, algorithm: Algorithm) -> Result<()>{
    //TODO: generate node tree, execute algorithm, create output file

    //Create the nodes that will be part of the graph/tree
    let nodes: Vec<_> = image
        .enumerate_pixels()
        .map(|(x, y, pixel)| {
            Node::new(*pixel, (x, y))
        })
        .collect();

    //Try to find the start of the maze
    let root = nodes
        .iter()
        .find(|node| node.borrow().color == Rgb::from(DEFAULT_STARTING_COLOR));

    //Check if there is actually a starting node
    match root{
        Some(root) => algorithm.execute(root, Rgb::from(DEFAULT_ENDING_COLOR))?,
        None => {
            return Err(
                Error::Generic(
                    format!("Couldn't find the starting point (the color should be {:?})", DEFAULT_STARTING_COLOR)
                )
            );
        }
    }

    Ok(())
}