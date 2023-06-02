use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub name_collection: String,
}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(bool)]
    IncludesAddress { address: String },
    #[returns(bool)]
    IncludesName { name: String },
}

#[cw_serde]
pub struct ConfigResponse {
    pub name_collection: String,
}
