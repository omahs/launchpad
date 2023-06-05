use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub whitelists: Vec<String>,
}

#[cw_serde]
pub enum ExecuteMsg {}

// TODO: autogenerate `IncludesAddress`
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},
    #[returns(bool)]
    IncludesAddress { address: String },
}
