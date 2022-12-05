use cosmwasm_schema::write_api;
use cw_bid::msg::{BidExecuteMsg, BidInstantiateMsg, BidMigrateMsg, BidQueryMsg};

fn main() {
    write_api! {
        instantiate: BidInstantiateMsg,
        execute: BidExecuteMsg,
        query: BidQueryMsg,
        migrate: BidMigrateMsg,
    }
}
