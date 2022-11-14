///Custom error type
#[derive(thiserror::Error, Debug)]
pub enum Error{
    #[error("Generic error: {0}")]
    _Generic(String),

    #[error("IO error: {0}")]
    IOError(String),
}