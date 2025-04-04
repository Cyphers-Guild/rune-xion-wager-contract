use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Game already exists")]
    DuplicateGame {},

    #[error("Game not found")]
    GameNotFound {},

    #[error("No address")]
    NoAddress {},
}
