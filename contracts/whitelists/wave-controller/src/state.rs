use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
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
