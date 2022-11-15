///Custom error type
#[derive(thiserror::Error, Debug)]
pub enum Error{
    #[error("Generic error: {0}")]
    Generic(String),

    
    #[error("Parsing error: {0}")]
    ParsingError(#[from] std::num::ParseIntError),
    

    #[error("IO error: {0}")]
    IOError(String),
}