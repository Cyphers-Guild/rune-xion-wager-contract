use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid winner")]
    InvalidWinner {},

    #[error("Invalid player")]
    InvalidPlayer {},

    #[error("Must send exactly one coin")]
    InsufficientAmount {},
    
    #[error("Game is not finished yet")]
    GameOngoing {},

    #[error("Game already resolved")]
    GameResolved {},

    #[error("Game is not finished yet")]
    GameNotFinished {},

    #[error("Waging is closed")]
    WagingClosed {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

}
