///Public reexports
pub use crate::error::Error;
pub use crate::utils::*;

///Redefinition of the result type
pub type Result<T> = core::result::Result<T, Error>;

//Useful color constants as tuples of u8
const _COLOR_WHITE: (u8, u8, u8) = (255, 255, 255);
const _COLOR_BLACK: (u8, u8, u8) = (0, 0, 0);
