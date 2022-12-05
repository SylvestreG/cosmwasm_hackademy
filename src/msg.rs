use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct BidInstantiateMsg {
    pub owner: Option<String>,
}

#[cw_serde]
pub struct BidExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum BidQueryMsg {
    #[returns(Addr)]
    GetOwner {},
}

#[cw_serde]
pub struct BidMigrateMsg {}
