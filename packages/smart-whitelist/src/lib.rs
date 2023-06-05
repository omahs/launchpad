use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{to_binary, Addr, QuerierWrapper, QueryRequest, StdResult, WasmQuery};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(bool)]
    IncludesAddress { address: String },
}

#[cw_serde]
pub struct SmartWhitelistContract(pub Addr);

impl SmartWhitelistContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn includes(&self, querier: &QuerierWrapper, address: String) -> StdResult<bool> {
        let includes: bool = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&QueryMsg::IncludesAddress { address })?,
        }))?;
        Ok(includes)
    }

    // TODO: add the other helpers
}
