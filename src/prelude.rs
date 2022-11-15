///Public reexports
pub use crate::error::Error;
pub use crate::utils::*;

///Redefinition of the result type
pub type Result<T> = core::result::Result<T, Error>;

//Useful color constants as arrays of u8
pub const _COLOR_WHITE: [u8; 3] = [255, 255, 255];
pub const _COLOR_BLACK: [u8; 3] = [0, 0, 0];

pub const COLOR_GREEN: [u8; 3] = [0, 255, 0];
pub const COLOR_RED:   [u8; 3] = [255, 0, 0];


pub const DEFAULT_STARTING_COLOR: [u8; 3] = COLOR_GREEN;
pub const DEFAULT_ENDING_COLOR: [u8; 3] = COLOR_RED;