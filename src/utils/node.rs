use std::rc::Rc;
use std::cell::RefCell;

///Building block of a graph/tree structure
#[derive(Debug, Clone)]
pub struct Node{
    color: (u8, u8, u8),
    coords: (u64, u64),
    seen: bool,
    distance: Distance,
    edges: Vec<Rc<RefCell<Node>>>,
}

impl Node{
    pub fn new(color: (u8, u8, u8), coords: (u64, u64)) -> Self{
        Node {
            color,
            coords,
            seen: true,
            distance: Distance::None,
            edges: Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
enum Distance{
    Value(u64),
    None
}