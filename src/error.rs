
#[derive(thiserror::Error, Debug)]
pub enum Error{
    #[error("Generic error: `{0}`")]
    Generic(String),
}