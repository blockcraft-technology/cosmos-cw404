use cosmwasm_std::Addr;
use cw_storage_plus::Map;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FractionalNFT {
    pub owner: Addr,            // originl owner
    pub nft_contract: Addr,     // nft contract addr
    pub token_id: String,       // nft id    
    pub total_shares: Uint128,  // fraction counter
    pub cw20_contract: Addr,    // fraction contract
}

pub const FRACTIONAL_NFTS: Map<(&Addr, &str), FractionalNFT> = Map::new("fractional_nfts");
