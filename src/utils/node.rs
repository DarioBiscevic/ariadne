use image::Rgb;

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

///Building block of a graph/tree structure.
#[derive(Debug, Clone)]
pub struct Node{
    pub color: Rgb<u8>,
    pub node_type: NodeType,
    pub coords: (u32, u32),
    pub seen: bool,
    pub distance: Distance,
    pub previous: Option<Rc<RefCell<Node>>>,
    pub edges: Vec<Rc<RefCell<Node>>>,
}

impl Node{
    ///Creates an `Rc<RefCell<Node>>` pointer to a new node.
    pub fn new(color: Rgb<u8>, coords: (u32, u32), node_type: NodeType) -> Rc<RefCell<Self>>{
        Rc::new(RefCell::new(Node {
            color,
            node_type,
            coords,
            seen: false,
            previous: None,
            distance: Distance::Infinity,
            edges: Vec::new()
        }))
    }

    ///Sets the node's distance to `val`.
    pub fn set_distance(&mut self, val: u64){
        self.distance = Distance::Value(val);
    }

    ///Checks if the `other` node is directly neighbouring with the current node.
    pub fn is_neighbour_to(&self, other: &Rc<RefCell<Self>>) -> bool{
        let other_x = other.borrow().coords.0;
        let other_y = other.borrow().coords.1;

        let diff_x = other_x as i64 - self.coords.0 as i64;
        let diff_y = other_y as i64 - self.coords.1 as i64;

        (diff_x == 0 && (diff_y == 1 || diff_y == -1)) || (diff_y == 0 && (diff_x == 1 || diff_x == -1))
    }

    ///Returns `true` if the node is a "road node".
    pub fn _is_road(&self) -> bool{
        self.node_type == NodeType::Road
    }

    ///Returns `true` if the node is a "wall node".
    pub fn _is_wall(&self) -> bool{
        self.node_type == NodeType::Wall
    }

    ///Returns `true` if the node is a starting node.
    pub fn is_start(&self) -> bool{
        self.node_type == NodeType::Start
    }

    ///Returns `true` if the node is an ending node.
    pub fn is_end(&self) -> bool{
        self.node_type == NodeType::End
    }
}

impl PartialEq for Node{
    fn eq(&self, other: &Self) -> bool {
        if let Distance::Value(curr_dist) = self.distance{
            match other.distance {
                Distance::Value(other_dist) => curr_dist == other_dist,
                Distance::Infinity => false
            }
        }else{
            match other.distance{
                Distance::Value(_) => false,
                Distance::Infinity => true,
            }
        }
    }
}

impl Eq for Node{}


///Implementation of the `PartialOrd` trait.
impl PartialOrd for Node{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Distance::Value(curr_dist) = self.distance{
            match other.distance {
                Distance::Value(other_dist) => Some(curr_dist.cmp(&other_dist)),
                Distance::Infinity => Some(Ordering::Less)
            }
        }else{
            match other.distance {
                Distance::Value(_) => Some(Ordering::Greater),
                Distance::Infinity => Some(Ordering::Equal)
            }
        }
    }
}

///Implementation of the `Ord` trait.
impl Ord for Node{
    fn cmp(&self, other: &Self) -> Ordering {
        if let Distance::Value(curr_dist) = self.distance{
            match other.distance {
                Distance::Value(other_dist) => curr_dist.cmp(&other_dist),
                Distance::Infinity => Ordering::Less
            }
        }else{
            match other.distance {
                Distance::Value(_) => Ordering::Greater,
                Distance::Infinity => Ordering::Equal
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Distance{
    Value(u64),
    Infinity
}

///Enum to make the identification of the node type easier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType{
    Wall,
    Road,
    Start,
    End
}