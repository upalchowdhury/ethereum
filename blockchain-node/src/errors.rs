use thiserror::Error;
use eyre::Report;



#[derive(Debug, Error)]
pub enum NodeError {
    #[error("execution error: {0}")]
    ExecutionError(Error),

    #[error("payload error: {0}")]
    PayloadError(Report),

    #[error(transparent)]
    BlockNotFoundError(#[from] Error),


}

impl NodeError {
    pub fn jsonrpsee_error(self) -> jsonrpsee::core::Error {
        match self {
            NodeError::ExecutionError(data) => {
                let mut msg = "execution reverted".to_string();
                if let Some(reason) = data.as_ref.and_then(Error) {
                    msg = format!("{msg}:{reason}")
                }

                jsonrpsee::core::Error::Call(jsonrpsee::types::error::CallError::Custom(
                        jsonrpsee::types::error::ErrorObject::owned(
                            3,
                            msg,
                            data.map(|data| format!("0x{}", hex::encode(data))),
                        ),
                    )
                )
            }
        }
    }
}