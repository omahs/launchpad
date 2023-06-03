use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Coin, QuerierWrapper, QueryRequest, StdResult, WasmQuery};
use cw_storage_plus::Map;

#[cw_serde]
pub enum WhitelistType {
    Immutable,
    Mutable,
    Captcha,
    NftOwner,
    TwitterVerifiedName,
    Open,
    Custom(String),
}

#[cw_serde]
pub struct WhitelistConfig {
    pub whitelist_type: WhitelistType,
    pub mint_allowance: u32, // per address limit
    pub mint_price: Coin,
    pub start_time: u64,
    pub end_time: u64,
}

pub const WHITELISTS: Map<Addr, WhitelistConfig> = Map::new("wls");

pub const MINTED_LIST: Map<Addr, u64> = Map::new("ml");

#[cw_serde]
pub struct WhitelistContract(pub Addr);

impl WhitelistContract {
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
