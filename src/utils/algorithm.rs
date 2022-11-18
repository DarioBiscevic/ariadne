use std::rc::Rc;
use std::cell::RefCell;

use crate::prelude::*;
use super::{Node, Distance};

///Enums of possible algorithms to use.
/// In the future, other pathfinding algorithms will be added.
#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Algorithm{
    Dijkstra
}

impl Algorithm{
    ///Execute the pathfinding algorithm
    pub fn execute(&self, nodes: Vec<Rc<RefCell<Node>>>) -> Result<Path>{
        match self{
            Self::Dijkstra => dijkstra(nodes),
        }
    }
}

fn dijkstra(nodes: Vec<Rc<RefCell<Node>>>) -> Result<Path>{
    //Try to find the start of the maze
    let maybe_root = nodes
        .iter()
        .find(|node| node.as_ref().borrow().is_start());

    //Check if there is actually a starting node
    let root = match maybe_root{
        Some(root) => root,
        None => {
            return Err(
                Error::Generic(format!("Couldn't find the starting point (the color should be {:?})", DEFAULT_STARTING_COLOR))
            );
        }
    };

    //Initialize the start of the tree
    root.borrow_mut().set_distance(0);

    let mut ending = None;

    //Main loop of the algorithm: it continues while there are unseen nodes and if the target isn't found
    while nodes.iter().any(|n| !n.borrow().seen) {
        let current_ptr = nodes.iter().filter(|n| !n.borrow().seen).min();
        
        let mut current = match current_ptr{
            Some(node) => node.borrow_mut(),
            None => return Err(Error::Generic("Didn't find a valid vertex".to_string()))
        };

        //Exit the loop if the target is found
        if current.is_end(){
            ending = Some(current.clone());
            break;
        }

        //Mark the current node as seen
        current.seen = true;

        //Iterate through the neighbours and calculate their tentative distance
        for neighbour in current.edges.iter().filter(|n| !n.borrow().seen){
            let mut neighbour_mut = neighbour.borrow_mut();
        
            //First check if the tentative distance of the current node is less than infinity
            let curr_dist = match current.distance{
                Distance::Value(dist) => dist,
                Distance::Infinity => {
                    return Err(Error::Generic(
                            format!("Current node has value set to infinity: (x,y) = {:?}, color: {:?}", current.coords, current.color)
                        ))
                }
            };
            

            if let Distance::Value(neighbour_dist) = neighbour_mut.distance{
                let new_distance = curr_dist + 1;
    
                //Update neighbour's tentative distance
                if new_distance < neighbour_dist{
                    neighbour_mut.distance = Distance::Value(new_distance);

                    //Update the neighbour's parent node 
                    neighbour_mut.previous = Some(Rc::new(RefCell::new(current.clone())));
                }
            }else{
                //If the neighbours distance is infinity, setup the new finite value
                neighbour_mut.distance = Distance::Value(curr_dist + 1);
                neighbour_mut.previous = Some(Rc::new(RefCell::new(current.clone())));
            }
        }
    }

    //Check if the ending node is actually the target
    match ending{
        Some(finish) => {
            //If the ending node is the target, "compile" the path into a vector
            if finish.is_end() && finish.previous.is_some(){
                //Accumulator for each step of the path
                let mut path = Vec::new();
                let mut temp = finish;

                //Loop backwards until the starting node is reached
                while temp.previous.is_some() {
                    path.push(temp.coords);
                    let previous = temp.previous.unwrap();
                    temp = previous.borrow().clone();
                }
                
                path.reverse();

                return Ok(Path::Found(path))
            }

            Ok(Path::NotFound)
        },
        None => Ok(Path::NotFound)
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Path{
    NotFound,
    Found(Vec<(u32, u32)>)
}