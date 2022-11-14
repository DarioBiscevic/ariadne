pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

const COLOR_WHITE: (u8, u8, u8) = (255, 255, 255);
const COLOR_BLACK: (u8, u8, u8) = (0, 0, 0);