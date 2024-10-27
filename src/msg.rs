use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    FractionalizeNFT {
        nft_contract: String,
        token_id: String,
        total_shares: Uint128,
        token_name: String,
        token_symbol: String,
    },
    RedeemNFT {
        nft_contract: String,
        token_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    FractionalNFTInfo {
        nft_contract: String,
        token_id: String,
    },
    FractionalTokenAddress {
        nft_contract: String,
        token_id: String,
    },
}
