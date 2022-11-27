use std::rc::Rc;
use std::cell::RefCell;

use crate::prelude::*;
use super::{Node, Distance};

///Enums of possible algorithms to use.
/// In the future, other pathfinding algorithms will be added.
#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Algorithm{
    Dijkstra,
    AStar
}

impl Algorithm{
    ///Execute the pathfinding algorithm
    pub fn execute(&self, root: &Rc<RefCell<Node>>, n_nodes: usize) -> Result<Path>{
        match self{
            Self::Dijkstra => dijkstra(root, n_nodes),
            Self::AStar    => a_star(root, n_nodes),
        }
    }
}

fn dijkstra(root: &Rc<RefCell<Node>>, n_nodes: usize) -> Result<Path>{
    //Initialize the start of the tree
    root.borrow_mut().set_distance(0);

    let mut path_edges: Vec<Rc<RefCell<Node>>> = Vec::with_capacity(n_nodes);
    path_edges.push(root.clone());

    let mut ending = None;

    //Loop while there are nodes (and subsequent paths) to expand
    while !path_edges.is_empty(){

        //Remove all the nodes that were already checked - they won't be used again
        path_edges.retain(|n| !n.borrow().seen);

        //Sort the vector and get the node with the smallest tentative distance
        path_edges.sort();
        let current = path_edges[0].clone();

        let mut current = current.borrow_mut();

        //Exit the loop if the target is found
        if current.is_end(){
            ending = Some(current.clone());
            break;
        }

        //Mark the current node as seen
        current.seen = true;

        //First check if the tentative distance of the current node is less than infinity
        let curr_dist = match current.distance{
            Distance::Value(dist) => dist,
            Distance::Infinity => {
                return Err(Error::Generic(
                        format!("Current node has value set to infinity: (x,y) = {:?}, color: {:?}", current.coords, current.color)
                    ))
            }
        };

        //Iterate through the neighbours and calculate their tentative distance
        for neighbour in current.edges.iter().filter(|n| !n.borrow().seen){
            let mut neighbour_mut = neighbour.borrow_mut();
            
            //Calculate the new tentative distance (1 "unit" is the distance between 2 pixels)
            let new_distance = curr_dist + 1;
            
            if let Distance::Value(neighbour_dist) = neighbour_mut.distance{
                //Update neighbour's tentative distance
                if new_distance < neighbour_dist{
                    neighbour_mut.distance = Distance::Value(new_distance);
                }
            }else{
                //If the neighbours distance is infinity, setup the new finite value
                neighbour_mut.distance = Distance::Value(new_distance);
            }

            //Update the neighbour's parent node 
            neighbour_mut.previous = Some(Rc::new(RefCell::new(current.clone())));

            //Add the neighbour to the set of edges to expand
            path_edges.push(neighbour.clone());
        }
    }

    //Check if the ending node is actually the target
    match ending{
        Some(finish) => {
            //If the ending node is the target, "compile" the path into a vector
            if finish.is_end() && finish.previous.is_some(){
                //Accumulator for each step of the path
                let mut path = Vec::new();
                let mut current = finish;

                //Loop backwards until the starting node is reached
                while let Some(previous) = current.previous {
                    path.push(current.coords);
                    current = previous.borrow().clone();
                }
                
                path.reverse();

                return Ok(Path::Found(path))
            }

            Ok(Path::NotFound)
        },
        None => Ok(Path::NotFound)
    }
}

fn a_star(root: &Rc<RefCell<Node>>, n_nodes: usize) -> Result<Path>{
    //Initialize the start of the tree
    root.borrow_mut().set_distance(0);

    let mut path_edges: Vec<Rc<RefCell<Node>>> = Vec::with_capacity(n_nodes);
    path_edges.push(root.clone());

    let mut ending = None;

    //Loop while there are nodes (and subsequent paths) to expand
    while !path_edges.is_empty(){
        //Remove all the nodes that were already checked - they won't be used again
        path_edges.retain(|n| !n.borrow().seen);

        //Sort the vector and get the node with the smallest tentative distance
        path_edges.sort();
        let current = path_edges[0].clone();

        let mut current = current.borrow_mut();

        //Exit the loop if the target is found
        if current.is_end(){
            ending = Some(current.clone());
            break;
        }

        //Mark the current node as seen
        current.seen = true;

        //First check if the tentative distance of the current node is less than infinity
        let curr_dist = match current.distance{
            Distance::Value(dist) => dist,
            Distance::Infinity => {
                return Err(Error::Generic(
                        format!("Current node has value set to infinity: (x,y) = {:?}, color: {:?}", current.coords, current.color)
                    ))
            }
        };

        //Iterate through the neighbours and calculate their tentative distance
        for neighbour in current.edges.iter().filter(|n| !n.borrow().seen){
            let mut neighbour_mut = neighbour.borrow_mut();
            
            //Calculate the new tentative distance + the heuristic distance
            let new_distance = curr_dist + 1 + current.heuristic;
            
            if let Distance::Value(neighbour_dist) = neighbour_mut.distance{
                //Update neighbour's tentative distance
                if new_distance < neighbour_dist{
                    neighbour_mut.distance = Distance::Value(new_distance);
                }
            }else{
                //If the neighbours distance is infinity, setup the new finite value
                neighbour_mut.distance = Distance::Value(new_distance);
            }

            //Update the neighbour's parent node 
            neighbour_mut.previous = Some(Rc::new(RefCell::new(current.clone())));

            //Add the neighbour to the set of edges to expand
            path_edges.push(neighbour.clone());
        }
    }

    //Check if the ending node is actually the target
    match ending{
        Some(finish) => {
            //If the ending node is the target, "compile" the path into a vector
            if finish.is_end() && finish.previous.is_some(){
                //Accumulator for each step of the path
                let mut path = Vec::new();
                let mut current = finish;

                //Loop backwards until the starting node is reached
                while let Some(previous) = current.previous {
                    path.push(current.coords);
                    current = previous.borrow().clone();
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