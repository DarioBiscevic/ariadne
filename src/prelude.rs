///Public reexports
pub use crate::error::Error;
pub use crate::utils::*;

///Redefinition of the result type.
pub type Result<T> = core::result::Result<T, Error>;

pub const DEFAULT_OUTPUT_NAME: &str = "output.png";

//Useful color constants as arrays of u8
pub const COLOR_WHITE: &[u8] = &[255, 255, 255];
pub const COLOR_BLACK: &[u8] = &[0, 0, 0];

pub const COLOR_RED:   &[u8] = &[255, 0, 0];
pub const COLOR_GREEN: &[u8] = &[0, 255, 0];
pub const _COLOR_BLUE: &[u8] = &[0, 0, 255];
pub const COLOR_ORANGE:&[u8] = &[255, 91, 0];
pub const COLOR_YELLOW:&[u8] = &[255, 255, 0];


pub const DEFAULT_STARTING_COLOR: &[u8] = COLOR_GREEN;
pub const DEFAULT_ENDING_COLOR:   &[u8] = COLOR_RED;

pub const DEFAULT_ROAD_COLOR: &[u8] = COLOR_WHITE;
pub const DEFAULT_WALL_COLOR: &[u8] = COLOR_BLACK;

pub const DEFAULT_PATH_COLOR: &[u8] = COLOR_ORANGE;
pub const DEFAULT_SEEN_COLOR: &[u8] = COLOR_YELLOW;