use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Coin, QuerierWrapper, QueryRequest, StdResult, WasmQuery};
use cw_storage_plus::Map;

#[cw_serde]
pub struct WhitelistConfig {
    pub start_time: u64,
    pub end_time: u64,
    pub mint_allowance: u32, // per address limit
    pub mint_price: Coin,
    pub max_mint_count: u64,
}

#[cw_serde]
pub struct WhitelistData {
    pub config: WhitelistConfig,
    pub mint_count: u64,
}

pub const WHITELISTS: Map<Addr, WhitelistData> = Map::new("wls");

pub const MINTED_LIST: Map<Addr, u64> = Map::new("ml");

#[cw_serde]
pub struct SmartWhitelistContract(pub Addr);

impl SmartWhitelistContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn includes(&self, querier: &QuerierWrapper, address: String) -> StdResult<bool> {
        let includes: bool = querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&sg_smart_whitelist::QueryMsg::IncludesAddress { address })?,
        }))?;
        Ok(includes)
    }

    // TODO: add the other helpers
}
