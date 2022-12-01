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
    pub g_score: u64,
    pub heuristic: u64,
    pub f_score: u64,
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
            g_score: u64::MAX,
            heuristic: u64::MAX,
            f_score: u64::MAX,
            edges: Vec::with_capacity(4)
        }))
    }

    ///Sets the node's heuristic distance (manhattan geometry) from the specified `target`.
    pub fn set_heuristic_distance_from(&mut self, target: (u32, u32)){
        self.heuristic = (self.coords.0.abs_diff(target.0) + self.coords.1.abs_diff(target.1)) as u64;
    }

    ///Returns the coordinates of the possible neighbours.
    pub fn neighbouring_coords(coords: (u32, u32)) -> Vec<(u32, u32)>{
        let (x, y) = coords;
        let mut out = Vec::new();

        if x < u32::MAX - 1{
            out.push((x + 1, y));
        }

        if x > 0{
            out.push((x - 1, y));
        }

        if y < u32::MAX - 1{
            out.push((x, y + 1));
        }

        if y > 0{
            out.push((x, y - 1));
        }

        out
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
        self.f_score == other.f_score
    }
}

impl Eq for Node{}

///Implementation of the `PartialOrd` trait.
impl PartialOrd for Node{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

///Implementation of the `Ord` trait.
impl Ord for Node{
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.cmp(&other.f_score)
    }
}
///Enum to make the identification of the node type easier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeType{
    Wall,
    Road,
    Start,
    End
}