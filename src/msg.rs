use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct BidInstantiateMsg {
    pub owner: Option<String>,
    pub denom: Option<String>,
}

#[cw_serde]
pub enum BidExecuteMsg {
    Bid {},
    Close {},
    Retract { receiver: Option<String> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum BidQueryMsg {
    #[returns(Addr)]
    GetOwner {},
    #[returns(String)]
    GetDenom {},
    #[returns(Uint128)]
    BidForAddress { address: String },
    #[returns(bool)]
    BidClosed {},
    #[returns(Uint128)]
    HighestBid {},
    #[returns(Option<Addr>)]
    HighestBidder {},
    #[returns(Option<Addr>)]
    BidWinner,
}

#[cw_serde]
pub struct BidMigrateMsg {}
