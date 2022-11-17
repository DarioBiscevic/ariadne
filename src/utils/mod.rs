use image::{RgbImage, Pixel, Rgb};

use std::rc::Rc;
use std::cell::RefCell;

pub mod algorithm;
mod node;

use crate::prelude::*;
use node::{Node, NodeType, Distance};

pub use crate::utils::algorithm::{Algorithm, Path};

///Entry point for the main process. It computes the path from the starting point to the ending point
/// using the specified `algorithm`.
pub fn run(image: RgbImage, algorithm: Algorithm) -> Result<()>{
    //Create the nodes that will be part of the graph/tree
    let nodes: Vec<Rc<RefCell<Node>>> = image
        .enumerate_pixels()
        .map(|(x, y, pixel)| {

            //Identify the node type
            let node_type = match pixel.channels(){
                DEFAULT_STARTING_COLOR => NodeType::Start,
                DEFAULT_ENDING_COLOR => NodeType::End,
                DEFAULT_ROAD_COLOR => NodeType::Road,
                DEFAULT_WALL_COLOR => NodeType::Wall,
                _ => NodeType::Road,
            };

            //Return a newly created node
            Node::new(*pixel, (x, y), node_type)
        })
        .collect();

    //Connect the nodes
    connect_nodes(&nodes);

    let result = algorithm.execute(nodes)?;

    match result {
        Path::Found(path) => {
            //Prepare buffer for the output image
            let mut out_img = RgbImage::new(image.width(), image.height());

            //Iterate through all the pixels and change only those that are parte of the path
            for (x, y, pixel) in image.enumerate_pixels(){
                if path.contains(&(x, y)){
                    //Convert the default path color from a slice to an array of values
                    let path_color = [DEFAULT_PATH_COLOR[0], DEFAULT_PATH_COLOR[1], DEFAULT_PATH_COLOR[2]];
                    out_img.put_pixel(x, y, Rgb::from(path_color));
                }else{
                    out_img.put_pixel(x, y, *pixel);
                }
            }

            //Save the output image
            out_img.save(DEFAULT_OUTPUT_NAME)?;
        },
        Path::NotFound => { eprintln!("Path not found!"); }
    }

    Ok(())
}

///Function that fills the `edges` property of every node with the appropriate
/// neighbouring nodes. Every node has 4 neighbours: up, down, left, right.
fn connect_nodes(nodes: &[Rc<RefCell<Node>>]){
    for node in nodes.iter().filter(|n| !n.as_ref().borrow().is_wall()){
        for neighbour in nodes.iter().filter(|n| !n.as_ref().borrow().is_wall() && node.as_ref().borrow().is_neighbour_to(n)){
            node.borrow_mut().edges.push(neighbour.clone());
        }
    }
}