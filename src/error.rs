use thiserror::Error;

pub type Result<T> = std::result::Result<T, EnigmaError>;

#[derive(Error, Debug)]
pub enum EnigmaError {
    #[error("Cannot process character {0}")]
    InvalidChar(char),
}
