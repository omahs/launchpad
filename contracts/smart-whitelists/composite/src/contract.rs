use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use sg_smart_whitelist::SmartWhitelistContract;

const CONTRACT_NAME: &str = "crates.io:sg-twitter-verified-name-whitelist";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let lists = msg
        .whitelists
        .iter()
        .map(|w| deps.api.addr_validate(w))
        .collect::<StdResult<Vec<_>>>()?;

    CONFIG.save(deps.storage, &Config { whitelists: lists })?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!("unsupported message")
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::IncludesAddress { address } => to_binary(&query_includes_address(deps, address)?),
    }
}

fn query_includes_address(deps: Deps, address: String) -> StdResult<bool> {
    let lists = CONFIG.load(deps.storage)?.whitelists;

    for list in lists {
        let whitelist = SmartWhitelistContract(list);
        let included = whitelist.includes(&deps.querier, address.clone())?;
        if included {
            return Ok(true);
        }
    }

    // TODO: return the whitelist address that contains the address for the post op?

    Ok(false)
}

pub fn query_config(deps: Deps) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}
