use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{WhitelistConfig, WhitelistContract, WHITELISTS};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Order, StdResult};
use cw2::set_contract_version;
use sg_std::Response;

const CONTRACT_NAME: &str = "crates.io:sg-wave-controller";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

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
        ExecuteMsg::AddWhitelist { contract, config } => {
            execute_add_whitelist(deps, env, info, contract, config)
        }
        ExecuteMsg::RemoveWhitelist { contract } => {
            execute_remove_whitelist(deps, env, info, contract)
        }
        ExecuteMsg::UpdateWhitelist { contract, config } => {
            execute_update_whitelist(deps, env, info, contract, config)
        }
        ExecuteMsg::ProcessAddress { whitelist, address } => {
            execute_process_address(deps, env, info, whitelist, address)
        }
    }
}

pub fn execute_add_whitelist(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract: String,
    config: WhitelistConfig,
) -> Result<Response, ContractError> {
    let whitelist_addr = deps.api.addr_validate(&contract)?;

    // config.validate()?;

    WHITELISTS.save(deps.storage, whitelist_addr, &config)?;

    Ok(Response::new()
        .add_attribute("action", "add_whitelist")
        .add_attribute("sender", info.sender))
}

pub fn execute_remove_whitelist(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract: String,
) -> Result<Response, ContractError> {
    let whitelist_addr = deps.api.addr_validate(&contract)?;

    WHITELISTS.remove(deps.storage, whitelist_addr);

    Ok(Response::new()
        .add_attribute("action", "remove_whitelist")
        .add_attribute("sender", info.sender))
}

pub fn execute_update_whitelist(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    contract: String,
    config: WhitelistConfig,
) -> Result<Response, ContractError> {
    let whitelist_addr = deps.api.addr_validate(&contract)?;

    // config.validate()?;

    // WHITELISTS.update(deps.storage, whitelist_addr, &config)?;

    Ok(Response::new()
        .add_attribute("action", "update_whitelist")
        .add_attribute("sender", info.sender))
}

pub fn execute_process_address(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    whitelist: String,
    address: String,
) -> Result<Response, ContractError> {
    let whitelist = WhitelistContract(deps.api.addr_validate(&whitelist)?);
    let is_included = whitelist.includes(&deps.querier, address)?;

    // TODO: if included, update accounting in mint list

    Ok(Response::new()
        .add_attribute("action", "process_address")
        .add_attribute("sender", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Wave {} => to_binary(&query_wave(deps)?),
        QueryMsg::WhitelistConfig { contract } => {
            let whitelist_addr = deps.api.addr_validate(&contract)?;
            let config = WHITELISTS.load(deps.storage, whitelist_addr)?;
            to_binary(&config)
        }
        QueryMsg::CanMint {
            whitelist,
            address,
            count,
        } => {
            let whitelist_addr = deps.api.addr_validate(&whitelist)?;
            let whitelist = WhitelistContract(whitelist_addr);
            let is_included = whitelist.includes(&deps.querier, address)?;
            // TODO: check if not minted over max mint allowance
            to_binary(&is_included)
        }
    }
}

pub fn query_wave(deps: Deps) -> StdResult<Vec<String>> {
    let wave = WHITELISTS
        .keys(deps.storage, None, None, Order::Ascending)
        .map(|addr| addr.unwrap().to_string())
        .collect::<Vec<_>>();
    Ok(wave)
}
