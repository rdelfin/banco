use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("Failed to spawn node: {0}")]
    FailedToSpawn(String),
    #[error("Node already exists")]
    NodeAlreadyExists,
}

impl Error {
    pub fn from_spawn(err: std::io::Error) -> Error {
        Error::FailedToSpawn(err.to_string())
    }
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
