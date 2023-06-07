use cosmwasm_std::{entry_point, Deps};
use cosmwasm_std::{to_binary, Binary, DepsMut, Env, MessageInfo, StdResult};
use cw_multi_test::{Contract, ContractWrapper};
use sg4::MinterConfigResponse;
use sg_eth_airdrop::error::ContractError;
use sg_std::{Response, StargazeMsgWrapper};
use vending_factory::msg::VendingMinterCreateMsg;
use vending_minter::msg::{ExecuteMsg, QueryMsg};
use vending_minter::state::ConfigExtension;
use vending_minter::tests::mock_query_config;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: VendingMinterCreateMsg,
) -> Result<Response, ContractError> {
    let res = Response::new();
    Ok(res)
}

pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Err(ContractError::CollectionWhitelistMinterNotSet {})
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config()),
        _ => to_binary("invalid"),
    }
}

fn query_config() -> MinterConfigResponse<ConfigExtension> {
    mock_query_config()
}

pub fn mock_minter() -> Box<dyn Contract<StargazeMsgWrapper>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}
