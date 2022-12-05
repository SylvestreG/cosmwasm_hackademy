use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct BidMigrateMsg {}

#[cw_serde]
pub struct BidInstantiateMsg {}

#[cw_serde]
pub struct BidExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum BidQueryMsg {}
