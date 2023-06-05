use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    SmartWhitelistContract, WhitelistConfig, WhitelistData, MINTED_LIST, WHITELISTS,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    ensure, to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, StdResult,
};
use cw2::set_contract_version;
use cw_ownable::{assert_owner, get_ownership};
use sg_std::Response;

const CONTRACT_NAME: &str = "crates.io:sg-wave-controller";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&msg.owner))?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateOwnership(action) => update_ownership(deps, env, info, action),
        ExecuteMsg::AddWhitelist { contract, config } => {
            execute_add_whitelist(deps, env, info, contract, config)
        }
        ExecuteMsg::RemoveWhitelist { contract } => {
            execute_remove_whitelist(deps, env, info, contract)
        }
        ExecuteMsg::UpdateWhitelist { contract, config } => {
            execute_update_whitelist(deps, env, info, contract, config)
        }
        ExecuteMsg::PostMint { whitelist, address } => {
            execute_process_address(deps, env, info, whitelist, address)
        }
        ExecuteMsg::Purge {} => execute_purge(deps, env, info),
    }
}

/// Wraps around cw_ownable::update_ownership to extract the result and wrap it in a Stargaze Response
fn update_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: cw_ownable::Action,
) -> Result<Response, ContractError> {
    let ownership = cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
    Ok(Response::new().add_attributes(ownership.into_attributes()))
}

fn execute_purge(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;

    // iterate through each whitelist and called `purge()`
    let whitelists = WHITELISTS.keys(deps.storage, None, None, Order::Ascending);
    for list in whitelists {
        let list = list?;
        let contract_name = cw2::query_contract_info(&deps.querier, list.clone())?.contract;
        if !contract_name.contains("smart") {
            // TODO: this contract is in core and has to be merged in
            // let contract = MutableWhitelistContract(list);
            // contract.purge(&deps.querier)?;
        }
    }

    // TODO: purge the minted list
    // TODO: purge all whitelist data

    Ok(Response::new().add_attribute("action", "purge"))
}

fn execute_add_whitelist(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract: String,
    config: WhitelistConfig,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;

    let whitelist_addr = deps.api.addr_validate(&contract)?;

    // config.validate()?;
    let data = WhitelistData {
        config,
        mint_count: 0,
    };
    WHITELISTS.save(deps.storage, whitelist_addr, &data)?;

    Ok(Response::new()
        .add_attribute("action", "add_whitelist")
        .add_attribute("sender", info.sender))
}

fn execute_remove_whitelist(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract: String,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;

    let whitelist_addr = deps.api.addr_validate(&contract)?;

    WHITELISTS.remove(deps.storage, whitelist_addr);

    Ok(Response::new()
        .add_attribute("action", "remove_whitelist")
        .add_attribute("sender", info.sender))
}

fn execute_update_whitelist(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    contract: String,
    _config: WhitelistConfig,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;

    // let whitelist_addr = deps.api.addr_validate(&contract)?;

    // config.validate()?;

    // WHITELISTS.update(deps.storage, whitelist_addr, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_whitelist")
        .add_attribute("sender", info.sender))
}

fn execute_process_address(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    whitelist: String,
    address: String,
) -> Result<Response, ContractError> {
    let whitelist = SmartWhitelistContract(deps.api.addr_validate(&whitelist)?);
    let minting_address = deps.api.addr_validate(&address)?;

    ensure!(
        whitelist.includes(&deps.querier, address)?,
        ContractError::Unauthorized {}
    );

    // update mint count for the address
    let address_mint_count = MINTED_LIST
        .may_load(deps.storage, minting_address.clone())?
        .unwrap_or(0);
    MINTED_LIST.save(deps.storage, minting_address, &(address_mint_count + 1))?;

    // update mint count for the whitelist
    let whitelist_mint_count = WHITELISTS.load(deps.storage, whitelist.addr())?.mint_count;
    WHITELISTS.update(deps.storage, whitelist.addr(), |data| match data {
        Some(mut whitelist_data) => {
            whitelist_data.mint_count = whitelist_mint_count + 1;
            Ok(whitelist_data)
        }
        None => Err(ContractError::Unauthorized {}),
    })?;

    Ok(Response::new()
        .add_attribute("action", "process_address")
        .add_attribute("sender", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Ownership {} => to_binary(&get_ownership(deps.storage)?),
        QueryMsg::Wave {} => to_binary(&query_wave(deps)?),
        QueryMsg::WhitelistData { contract } => to_binary(&query_whitelist_data(deps, contract)?),
        QueryMsg::PreMint {
            whitelist,
            address,
            count,
        } => to_binary(&query_pre_mint(deps, whitelist, address, count)?),
    }
}

fn query_whitelist_data(deps: Deps, contract: String) -> StdResult<WhitelistData> {
    WHITELISTS.load(deps.storage, deps.api.addr_validate(&contract)?)
}

fn query_pre_mint(deps: Deps, whitelist: String, address: String, count: u32) -> StdResult<Coin> {
    let whitelist_addr = deps.api.addr_validate(&whitelist)?;
    let whitelist = SmartWhitelistContract(whitelist_addr);
    whitelist.includes(&deps.querier, address)?;

    // TODO: check if address not minted over max mint allowance
    // TODO: check if mint count for the whitelist is not over the max mint count for that list

    Ok(WHITELISTS
        .load(deps.storage, whitelist.addr())?
        .config
        .mint_price)
}

fn query_wave(deps: Deps) -> StdResult<Vec<String>> {
    let wave = WHITELISTS
        .keys(deps.storage, None, None, Order::Ascending)
        .map(|addr| addr.unwrap().to_string())
        .collect::<Vec<_>>();
    Ok(wave)
}
