use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::WhitelistConfig;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    AddWhitelist {
        contract: String,
        config: WhitelistConfig,
    },
    RemoveWhitelist {
        contract: String,
    },
    UpdateWhitelist {
        contract: String,
        config: WhitelistConfig,
    },
    ProcessAddress {
        whitelist: String,
        address: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(bool)]
    CanMint {
        whitelist: String,
        address: String,
        count: u32,
    },
    #[returns(Vec<String>)]
    Wave {},
    #[returns(WhitelistConfig)]
    WhitelistConfig { contract: String },
}
