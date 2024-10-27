use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("NFT not found")]
    NFTNotFound {},

    #[error("Fractional NFT not found")]
    FractionalNFTNotFound {},

    #[error("Insufficient fractional tokens")]
    InsufficientTokens {},

    #[error("Already fractionalized")]
    AlreadyFractionalized {},

    #[error("Cannot fractionalize zero shares")]
    ZeroShares {},

    #[error("CW20 Instantiate Failed")]
    CW20InstantiateFailed {},

    #[error("CW20 Hook Failed")]
    CW20HookFailed {},
}
