use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

use crate::prelude::*;
use super::Node;

///Enums of possible algorithms to use.
/// In the future, other pathfinding algorithms will be added.
#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Algorithm{
    Dijkstra,
    AStar,
    Dfs,
    Bfs,
}

impl Algorithm{
    ///Execute the pathfinding algorithm
    pub fn execute(&self, root: &Rc<RefCell<Node>>, n_nodes: usize) -> Result<Path>{
        match self{
            Self::Dijkstra => dijkstra(root, n_nodes),
            Self::AStar    => a_star(root, n_nodes),
            Self::Dfs      => dfs(root, n_nodes),
            Self::Bfs      => bfs(root, n_nodes),
        }
    }
}

fn dijkstra(root: &Rc<RefCell<Node>>, n_nodes: usize) -> Result<Path>{
    //Initialize the start of the tree
    root.borrow_mut().f_score = 0;

    //Array with the visitable edges
    let mut path_edges: VecDeque<Rc<RefCell<Node>>> = VecDeque::with_capacity(n_nodes);
    path_edges.push_front(root.clone());

    let mut ending = None;

    //Loop while there are nodes (and subsequent paths) to expand
    while let Some(current_rc) = path_edges.pop_front(){
        {
            let mut current = current_rc.borrow_mut();

            //Exit the loop if the target is found
            if current.is_end(){
                ending = Some(current.clone());
                break;
            }

            if current.seen{
                continue;
            }

            //Mark the current node as seen
            current.seen = true;
        }

        let current = current_rc.borrow();

        //Iterate through the neighbours
        for neighbour_rc in current.edges.iter().filter(|n| !n.borrow().seen){
            //Calculate the new tentative distance (1 "unit" is the distance between 2 pixels)
            let new_distance = current.f_score + 1;
            
            //Update neighbour's tentative distance if the current path is better than the previous
            if new_distance < neighbour_rc.borrow().f_score{

                {
                    let mut neighbour = neighbour_rc.borrow_mut();

                    neighbour.f_score = new_distance;

                    //Update the neighbour's parent node 
                    neighbour.previous = Some(current_rc.clone());
                }
                            
            }
            //Add the neighbour to the set of visitable edges
            let pos = path_edges.binary_search(neighbour_rc).unwrap_or_else(|e| e);
            path_edges.insert(pos, neighbour_rc.clone());    
        }
    }

    prepare_path(ending)
}

fn a_star(root: &Rc<RefCell<Node>>, n_nodes: usize) -> Result<Path>{
    //Initialize the start of the tree
    {
        let mut root_mut = root.borrow_mut();
        root_mut.f_score = root_mut.heuristic;
        root_mut.g_score = 0;
    }

    //Array with the visitable edges
    let mut path_edges: VecDeque<Rc<RefCell<Node>>> = VecDeque::with_capacity(n_nodes);
    path_edges.push_front(root.clone());

    let mut ending = None;

    //Loop while there are nodes (and subsequent paths) to expand
    while let Some(current_rc) = path_edges.pop_front(){
        {
            let mut current = current_rc.borrow_mut();

            //Exit the loop if the target is found
            if current.is_end(){
                ending = Some(current.clone());
                break;
            }

            if current.seen{
                continue;
            }

            //Mark the current node as seen
            current.seen = true;
        }    

        let current = current_rc.borrow();

        //Iterate through the neighbours
        for neighbour_rc in current.edges.iter().filter(|n| !n.borrow().seen){
            //Calculate the new tentative distance (1 "unit" is the distance between 2 pixels)
            let new_distance = current.g_score + 1;
            
            //Update neighbour's tentative distance if the current path is better than the previous
            if new_distance < neighbour_rc.borrow().g_score{
                {
                    let mut neighbour = neighbour_rc.borrow_mut();

                    //Update the score taking into account the heuristic
                    neighbour.g_score = new_distance;
                    neighbour.f_score = new_distance + neighbour.heuristic; 

                    //Update the neighbour's parent node 
                    neighbour.previous = Some(current_rc.clone());
                }                
            }
            //Add the neighbour to the set of visitable edges
            let pos = path_edges.binary_search(neighbour_rc).unwrap_or_else(|e| e);
            path_edges.insert(pos, neighbour_rc.clone());
        }
    }

    prepare_path(ending)
}

fn dfs(root: &Rc<RefCell<Node>>, n_nodes: usize) -> Result<Path>{

    //Stack with the "opened" vertices
    let mut stack = Vec::with_capacity(n_nodes);

    let mut ending = None;

    stack.push(root.clone());

    while let Some(node) = stack.pop(){
        let mut mut_node = node.borrow_mut();

        if mut_node.is_end(){
            ending = Some(mut_node.clone());
            break;
        }
        
        mut_node.seen = true;

        //Add the neighbouring vertices to the stack
        for neighbour in mut_node.edges.iter().filter(|n| !n.borrow().seen){
            let mut n = neighbour.borrow_mut();
            n.previous = Some(node.clone());

            stack.push(neighbour.clone());
        }
    }

    prepare_path(ending)
}

fn bfs(root: &Rc<RefCell<Node>>, n_nodes: usize) -> Result<Path>{

    //Queue with the "opened" vertices
    let mut queue = VecDeque::with_capacity(n_nodes);

    let mut ending = None;

    root.borrow_mut().seen = true;
    queue.push_back(root.clone());

    while !queue.is_empty() && ending.is_none(){
        let node_ref = queue.pop_front().unwrap();
        let node = node_ref.borrow_mut();

        if node.is_end(){
            ending = Some(node.clone());
        }

        //Add the neighbouring vertices to the stack
        for neighbour in node.edges.iter(){
            let mut mut_n = neighbour.borrow_mut();

            if !mut_n.seen{
                mut_n.seen = true;
                mut_n.previous = Some(node_ref.clone());
                queue.push_back(neighbour.clone());
            }
        }
    }
    
    prepare_path(ending)
}

fn prepare_path(ending: Option<Node>) -> Result<Path>{
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