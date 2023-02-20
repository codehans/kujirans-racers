use cosmwasm_std::StdError;
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("InvalidConfig")]
    InvalidConfig {},
    #[error("GameStarted")]
    GameStarted {},
    #[error("GameNotStarted")]
    GameNotStarted {},
    #[error("GameFull")]
    GameFull {},
    #[error("InvalidEntryFee")]
    InvalidEntryFee {},
    #[error("InvalidGame")]
    InvalidGame {},
}
