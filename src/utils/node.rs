use image::Rgb;

use std::rc::Rc;
use std::cell::RefCell;

///Building block of a graph/tree structure
#[derive(Debug, Clone)]
pub struct Node{
    pub color: Rgb<u8>,
    pub coords: (u32, u32),
    pub seen: bool,
    pub distance: Distance,
    pub edges: Vec<Rc<RefCell<Node>>>,
}

impl Node{
    pub fn new(color: Rgb<u8>, coords: (u32, u32)) -> Rc<RefCell<Self>>{
        Rc::new(RefCell::new(Node {
            color,
            coords,
            seen: true,
            distance: Distance::None,
            edges: Vec::new()
        }))
    }
}

#[derive(Debug, Clone)]
pub enum Distance{
    _Value(u64),
    None
}