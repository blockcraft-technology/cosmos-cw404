// contract.rs
use cosmwasm_std::{
    attr, entry_point, to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, Uint128,
};
use cw20::{Cw20Coin, Cw20ExecuteMsg, Cw20ReceiveMsg, MinterResponse};
use cw721::Cw721ExecuteMsg;

use crate::error::ContractError;
use crate::helpers::{
    instantiate_cw20, transfer_nft_to_contract, transfer_nft_to_recipient,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{FractionalNFT, FRACTIONAL_NFTS};

const DECIMALS: u8 = 0; // For NFTs, we usually don't need decimals

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::FractionalizeNFT {
            nft_contract,
            token_id,
            total_shares,
            token_name,
            token_symbol,
        } => fractionalize_nft(
            deps,
            env,
            info,
            nft_contract,
            token_id,
            total_shares,
            token_name,
            token_symbol,
        ),
        ExecuteMsg::RedeemNFT {
            nft_contract,
            token_id,
        } => redeem_nft(deps, env, info, nft_contract, token_id),
    }
}

fn fractionalize_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    nft_contract: String,
    token_id: String,
    total_shares: Uint128,
    token_name: String,
    token_symbol: String,
) -> Result<Response, ContractError> {
    if total_shares.is_zero() {
        return Err(ContractError::ZeroShares {});
    }

    let nft_contract_addr = deps.api.addr_validate(&nft_contract)?;
    let owner = info.sender.clone();

    // Check if the NFT is already fractionalized
    let key = (&nft_contract_addr, &token_id);
    if FRACTIONAL_NFTS.may_load(deps.storage, key)?.is_some() {
        return Err(ContractError::AlreadyFractionalized {});
    }

    // Transfer NFT to custody
    let transfer_nft_msg = transfer_nft_to_contract(&nft_contract_addr, &token_id, &env.contract.address);

    // create new cw20 tokens
    let cw20_instantiate_msg = instantiate_cw20(
        cw20code,
        &token_name,
        &token_symbol,
        DECIMALS,
        vec![Cw20Coin {
            address: owner.to_string(),
            amount: total_shares,
        }],
        None,
        Some(env.contract.address.to_string()),
        &format!("Fractional tokens for NFT {}:{}", nft_contract, token_id),
    );

    FRACTIONAL_NFTS.save(
        deps.storage,
        key,
        &FractionalNFT {
            owner: owner.clone(),
            nft_contract: nft_contract_addr.clone(),
            token_id: token_id.clone(),
            total_shares,
            cw20_contract: Addr::unchecked(""), 
        },
    )?;

    Ok(Response::new()
        .add_message(transfer_nft_msg)
        .add_message(cw20_instantiate_msg)
        .add_attributes(vec![
            attr("action", "fractionalize_nft"),
            attr("nft_contract", nft_contract),
            attr("token_id", token_id),
            attr("owner", owner),
            attr("total_shares", total_shares),
            attr("token_name", token_name),
            attr("token_symbol", token_symbol),
        ]))
}

fn redeem_nft(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    nft_contract: String,
    token_id: String,
) -> Result<Response, ContractError> {
    let nft_contract_addr = deps.api.addr_validate(&nft_contract)?;
    let sender = info.sender.clone();

    let key = (&nft_contract_addr, &token_id);
    let mut fractional_nft = FRACTIONAL_NFTS.load(deps.storage, key)?;

    // sender must have all the tokens
    let cw20_contract_addr = &fractional_nft.cw20_contract;

    // burn all tokens from the sender
    let burn_msg = WasmMsg::Execute {
        contract_addr: cw20_contract_addr.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::BurnFrom {
            owner: sender.to_string(),
            amount: fractional_nft.total_shares,
        })?,
        funds: vec![],
    };

    let transfer_nft_msg = transfer_nft_to_recipient(&nft_contract_addr, &token_id, &sender);

    FRACTIONAL_NFTS.remove(deps.storage, key);

    Ok(Response::new()
        .add_message(burn_msg)
        .add_message(transfer_nft_msg)
        .add_attributes(vec![
            attr("action", "redeem_nft"),
            attr("nft_contract", nft_contract),
            attr("token_id", token_id),
            attr("owner", sender),
        ]))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::FractionalNFTInfo {
            nft_contract,
            token_id,
        } => to_binary(&query_fractional_nft_info(deps, nft_contract, token_id)?),
        QueryMsg::FractionalTokenAddress {
            nft_contract,
            token_id,
        } => to_binary(&query_fractional_token_address(deps, nft_contract, token_id)?),
    }
}

fn query_fractional_nft_info(
    deps: Deps,
    nft_contract: String,
    token_id: String,
) -> StdResult<FractionalNFT> {
    let nft_contract_addr = deps.api.addr_validate(&nft_contract)?;
    let fractional_nft = FRACTIONAL_NFTS.load(deps.storage, (&nft_contract_addr, &token_id))?;
    Ok(fractional_nft)
}

fn query_fractional_token_address(
    deps: Deps,
    nft_contract: String,
    token_id: String,
) -> StdResult<String> {
    let nft_contract_addr = deps.api.addr_validate(&nft_contract)?;
    let fractional_nft = FRACTIONAL_NFTS.load(deps.storage, (&nft_contract_addr, &token_id))?;
    Ok(fractional_nft.cw20_contract.to_string())
}
