use cosmwasm_schema::cw_serde;
use cw4::MemberListResponse;

use cosmwasm_std::{
    to_json_binary, Addr, CosmosMsg, CustomQuery, Querier, QuerierWrapper, StdResult, WasmMsg,
    WasmQuery,
};

use crate::msg::{ExecuteMsg, QueryMsg};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[cw_serde]
pub struct SplitsContract(pub Addr);

impl SplitsContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    pub fn list_members<Q, T, CQ>(&self, querier: &Q) -> StdResult<MemberListResponse>
    where
        Q: Querier,
        T: Into<String>,
        CQ: CustomQuery,
    {
        let msg = QueryMsg::ListMembers {
            start_after: None,
            limit: None,
        };
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_json_binary(&msg)?,
        }
        .into();
        let res: MemberListResponse = QuerierWrapper::<CQ>::new(querier).query(&query)?;
        Ok(res)
    }
}
