use eyre::Report;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeError {

    #[error("Error serde: {0}")]
    SerdeError(String),

    #[error("Error converting a hex to U64: {0}")]
    HextoUError(String),

    #[error("Invalid transaction: {0}")]
    InvalidTransactionError(String),


    #[error("Trie structure error: {0}")]
    EthTrieError(String),
}

pub type Result<T> = std::result::Result<T, TypeError>;