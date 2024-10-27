use cosmwasm_std::{to_binary, Addr, CosmosMsg, WasmMsg};
use cw20::{Cw20ExecuteMsg, MinterResponse};
use cw721::Cw721ExecuteMsg;

use crate::msg::CW20InstantiateMsg;

pub fn transfer_nft_to_contract(
    nft_contract: &Addr,
    token_id: &str,
    contract_addr: &Addr,
) -> CosmosMsg {
    WasmMsg::Execute {
        contract_addr: nft_contract.to_string(),
        msg: to_binary(&Cw721ExecuteMsg::TransferNft {
            recipient: contract_addr.to_string(),
            token_id: token_id.to_string(),
        })
        .unwrap(),
        funds: vec![],
    }
    .into()
}

pub fn transfer_nft_to_recipient(
    nft_contract: &Addr,
    token_id: &str,
    recipient: &Addr,
) -> CosmosMsg {
    WasmMsg::Execute {
        contract_addr: nft_contract.to_string(),
        msg: to_binary(&Cw721ExecuteMsg::TransferNft {
            recipient: recipient.to_string(),
            token_id: token_id.to_string(),
        })
        .unwrap(),
        funds: vec![],
    }
    .into()
}

pub fn instantiate_cw20(
    code_id: u64,
    name: &str,
    symbol: &str,
    decimals: u8,
    initial_balances: Vec<cw20::Cw20Coin>,
    mint: Option<MinterResponse>,
    admin: Option<String>,
    label: &str,
) -> CosmosMsg {
    WasmMsg::Instantiate {
        admin,
        code_id,
        msg: to_binary(&CW20InstantiateMsg {
            name: name.to_string(),
            symbol: symbol.to_string(),
            decimals,
            initial_balances,
            mint,
        })
        .unwrap(),
        funds: vec![],
        label: label.to_string(),
    }
    .into()
}
