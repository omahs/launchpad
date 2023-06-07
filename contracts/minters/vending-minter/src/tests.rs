use cosmwasm_std::{Addr, Coin, Timestamp};
use sg4::{MinterConfig, MinterConfigResponse};

use crate::state::ConfigExtension;

// used to stub out the query_config function
// ex: eth airdrop tests
pub fn mock_query_config() -> MinterConfigResponse<ConfigExtension> {
    MinterConfigResponse {
        config: MinterConfig {
            factory: Addr::unchecked("some_factory".to_string()),
            collection_code_id: 4,
            mint_price: Coin::new(1000, "ustars"),
            extension: ConfigExtension {
                admin: Addr::unchecked("some_admin".to_string()),
                whitelist: Some(Addr::unchecked("contract2".to_string())),
                base_token_uri: "some_uri".to_string(),
                num_tokens: 5,
                start_time: Timestamp::from_seconds(30),
                per_address_limit: 5,
                payment_address: Some(Addr::unchecked("some_payment_address".to_string())),
                discount_price: Some(Coin::new(500, "ustars")),
            },
        },
        collection_address: "some_collection_address".to_string(),
    }
}
