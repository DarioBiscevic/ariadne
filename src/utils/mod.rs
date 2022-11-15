use image::{Rgb, RgbImage};

use std::rc::Rc;
use std::cell::RefCell;

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
    let nodes: Vec<Rc<RefCell<Node>>> = image
        .enumerate_pixels()
        .map(|(x, y, pixel)| {
            Node::new(*pixel, (x, y))
        })
        .collect();

    connect_nodes(&nodes);

    //Try to find the start of the maze
    let maybe_root = nodes
        .iter()
        .find(|node| node.as_ref().borrow().color == Rgb::from(DEFAULT_STARTING_COLOR));

    //Check if there is actually a starting node
    let root = match maybe_root{
        Some(root) => root,
        None => {
            return Err(
                Error::Generic(format!("Couldn't find the starting point (the color should be {:?})", DEFAULT_STARTING_COLOR))
            );
        }
    };

    algorithm.execute(root, Rgb::from(DEFAULT_ENDING_COLOR))?;

    Ok(())
}

///Function that fills the `edges` property of every node with the appropriate
/// neighbouring nodes. Every node has 4 neighbours: up, down, left, right.
fn connect_nodes(nodes: &Vec<Rc<RefCell<Node>>>){
    for node in nodes.iter().filter(|n| n.as_ref().borrow().color != Rgb::from(DEFAULT_WALL_COLOR)){
        for neighbour in nodes.iter().filter(|n| n.as_ref().borrow().color != Rgb::from(DEFAULT_WALL_COLOR) && node.as_ref().borrow().is_neighbour_to(*n)){
            node.borrow_mut().edges.push(neighbour.clone());
        }
    }
}