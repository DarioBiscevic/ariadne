///Custom error type
#[derive(thiserror::Error, Debug)]
pub enum Error{
    #[error("Generic error: {0}")]
    Generic(String),

    
    #[error("Parsing error: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
    
    #[error("Error saving output image: {0}")]
    SavingError(#[from] image::error::ImageError),

    #[error("IO error: {0}")]
    IOError(String),
}