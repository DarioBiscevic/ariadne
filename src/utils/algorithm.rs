use std::rc::Rc;
use std::cell::RefCell;

use crate::prelude::*;
use super::Node;

///Enums of possible algorithms to use.
/// In the future, other pathfinding algorithms will be added.
#[derive(clap::ValueEnum, Debug, Clone)]
pub enum Algorithm{
    Dijkstra
}

impl Algorithm{
    ///Execute the pathfinding algorithm
    pub fn execute(&self, graph: &Rc<RefCell<Node>>) -> Result<()>{
        match self{
            Self::Dijkstra => dijkstra(graph)
        }
    }
}

fn dijkstra(_graph: &Rc<RefCell<Node>>) -> Result<()>{

    Ok(())
}