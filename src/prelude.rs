///Public reexports
pub use crate::error::Error;
pub use crate::utils::*;

///Redefinition of the result type.
pub type Result<T> = core::result::Result<T, Error>;

//Useful color constants as arrays of u8
pub const COLOR_WHITE: &[u8] = &[255, 255, 255];
pub const COLOR_BLACK: &[u8] = &[0, 0, 0];

pub const COLOR_GREEN: &[u8] = &[0, 255, 0];
pub const COLOR_RED:   &[u8] = &[255, 0, 0];


pub const DEFAULT_STARTING_COLOR: &[u8] = COLOR_GREEN;
pub const DEFAULT_ENDING_COLOR:   &[u8] = COLOR_RED;

pub const DEFAULT_ROAD_COLOR: &[u8] = COLOR_WHITE;
pub const DEFAULT_WALL_COLOR: &[u8] = COLOR_BLACK;