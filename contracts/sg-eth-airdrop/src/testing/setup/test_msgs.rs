use cosmwasm_std::Addr;
use sg_multi_test::StargazeApp;

pub struct InstantiateParams<'a> {
    pub addresses: Vec<String>,
    pub funds_amount: u128,
    pub expected_airdrop_contract_id: u64,
    pub minter_address: Addr,
    pub admin_account: Addr,
    pub app: &'a mut StargazeApp,
    pub per_address_limit: u64,
}
