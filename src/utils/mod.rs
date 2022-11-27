use image::{RgbImage, Pixel, Rgb};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Instant;

pub mod algorithm;
pub mod args;
mod node;

use crate::prelude::*;
use node::{Node, NodeType, Distance};

pub use crate::utils::algorithm::{Algorithm, Path};
pub use args::Args;

///Entry point for the main process. It computes the path from the starting point to the ending point
/// using the specified `algorithm`.
pub fn run(mut image: RgbImage, arguments: Args) -> Result<()>{

    let start = Instant::now();
    let mut nodes: HashMap<(u32, u32), Rc<RefCell<Node>>> = HashMap::new();

    for (x, y, pixel) in image.enumerate_pixels().filter(|(_, _, pixel)| pixel.channels() != DEFAULT_WALL_COLOR){
        let node_type = match pixel.channels(){
            DEFAULT_STARTING_COLOR => NodeType::Start,
            DEFAULT_ENDING_COLOR => NodeType::End,
            DEFAULT_ROAD_COLOR => NodeType::Road,
            DEFAULT_WALL_COLOR => NodeType::Wall,
            _ => NodeType::Road,
        };

        nodes.insert((x, y), Node::new(*pixel, (x, y), node_type));
    }
    if arguments.logging{
        println!("Inserted {} nodes in {:?}", nodes.len(), start.elapsed());
    }

    //Try to find the end of the maze
    let maybe_end = nodes
        .iter()
        .find(|(_, node)| node.borrow().is_end());

    //Check if there is actually an ending node
    let end_coords = match maybe_end{
        Some((coords, _)) => coords,
        None => {
            return Err(
                Error::Generic(format!("Couldn't find the ending point (the color should be {:?})", DEFAULT_ENDING_COLOR))
            );
        }
    };

    let start = Instant::now();

    //Connect the nodes
    connect_nodes(&nodes, end_coords);

    if arguments.logging{
        println!("Time spent preparing the nodes: {:?}", start.elapsed());
    }

    //Try to find the start of the maze
    let maybe_root = nodes
        .iter()
        .find(|(_, node)| node.borrow().is_start());

    //Check if there is actually a starting node
    let root = match maybe_root{
        Some((_, root)) => root,
        None => {
            return Err(
                Error::Generic(format!("Couldn't find the starting point (the color should be {:?})", DEFAULT_STARTING_COLOR))
            );
        }
    };

    let start = Instant::now();
    let result = arguments.algorithm.execute(root, nodes.len())?;
    if arguments.logging{
        println!("Algorithm execution time: \t{:?}", start.elapsed());
    }

    match result {
        Path::Found(path) => {
            let start = Instant::now();

            //Convert the default path color from a slice to an array of values
            let path_color = Rgb::from([DEFAULT_PATH_COLOR[0], DEFAULT_PATH_COLOR[1], DEFAULT_PATH_COLOR[2]]);

            //Substitute the pixels of the path with the path color
            for (x, y) in path{
                image.put_pixel(x, y, path_color);
                if arguments.wider{
                    for (x_n, y_n) in Node::neighbouring_coords((x, y)){
                        if nodes.contains_key(&(x_n, y_n)){
                            image.put_pixel(x_n, y_n, path_color)
                        }
                    }
                }
            }
            
            //Save the output image
            image.save(arguments.output_file)?;

            if arguments.logging{
                println!("Output file creation time: \t{:?}", start.elapsed());
            }
        },
        Path::NotFound => { eprintln!("Path not found!"); }
    }

    Ok(())
}

///Function that fills the `edges` property of every node with the appropriate
/// neighbouring nodes. Every node has 4 neighbours: up, down, left, right.
fn connect_nodes(nodes: &HashMap<(u32, u32), Rc<RefCell<Node>>>, target: &(u32, u32)){
    for (_, node) in nodes.iter(){
        let mut mut_node = node.borrow_mut();
        mut_node.set_heuristic_distance_from(*target);

        for neighbour_coords in Node::neighbouring_coords(mut_node.coords){
            if let Some(neighbour) = nodes.get(&neighbour_coords){
                mut_node.edges.push(neighbour.clone());
            }
        }
    }
}