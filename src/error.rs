use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArmouryError {
    #[error("parse error")]
    ParseError(#[from] toml::de::Error),
    #[error("I/O error")]
    IoError(#[from] std::io::Error),
}
