
use cosmwasm_schema::write_api;
use cw_bid::msg::{BidInstantiateMsg, BidExecuteMsg, BidMigrateMsg, BidQueryMsg};

fn main() {
    write_api! {
        instantiate: BidInstantiateMsg,
        execute: BidExecuteMsg,
        query: BidQueryMsg,
        migrate: BidMigrateMsg,
    }
}
