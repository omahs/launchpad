use cosmwasm_schema::write_api;

use stargaze_smart_whitelist_twitter_verified_name::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
