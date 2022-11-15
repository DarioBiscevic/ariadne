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
    ///Creates an `Rc<RefCell<` pointer to a new node
    pub fn new(color: Rgb<u8>, coords: (u32, u32)) -> Rc<RefCell<Self>>{
        Rc::new(RefCell::new(Node {
            color,
            coords,
            seen: true,
            distance: Distance::None,
            edges: Vec::new()
        }))
    }

    ///Checks if the `other` node is directly neighbouring with the current node
    pub fn is_neighbour_to(&self, other: &Rc<RefCell<Self>>) -> bool{
        let other_x = other.borrow().coords.0;
        let other_y = other.borrow().coords.1;

        let diff_x = other_x as i64 - self.coords.0 as i64;
        let diff_y = other_y as i64 - self.coords.1 as i64;

        (diff_x == 0 && (diff_y == 1 || diff_y == -1)) || (diff_y == 0 && (diff_x == 1 || diff_x == -1))
    }
}

#[derive(Debug, Clone)]
pub enum Distance{
    _Value(u64),
    None
}