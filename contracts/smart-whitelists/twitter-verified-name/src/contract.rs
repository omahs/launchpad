use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use sg721_name::NameCollectionContract;

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

    let config = Config {
        name_collection: deps.api.addr_validate(&msg.name_collection)?,
    };
    CONFIG.save(deps.storage, &config)?;

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
        QueryMsg::IncludesAddress { address } => {
            to_binary(&query_twitter_verified_address(deps, address)?)
        }
        QueryMsg::IncludesName { name } => to_binary(&query_twitter_verified_name(deps, name)?),
    }
}

fn query_twitter_verified_name(deps: Deps, name: String) -> StdResult<bool> {
    NameCollectionContract(CONFIG.load(deps.storage)?.name_collection)
        .is_twitter_verified(&deps.querier, &name)
}

fn query_twitter_verified_address(deps: Deps, address: String) -> StdResult<bool> {
    let name_collection = NameCollectionContract(CONFIG.load(deps.storage)?.name_collection);
    let name = name_collection.name(&deps.querier, &address)?;

    name_collection.is_twitter_verified(&deps.querier, &name)
}

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        name_collection: config.name_collection.to_string(),
    })
}
