use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<Addr> = Item::new("OWNER");
pub const DENOM: Item<String> = Item::new("DENOM");

pub const BID_BY_ADDR: Map<Addr, Uint128> = Map::new("BID_BY_ADDR");
pub const BID_RETRACTED_FOR_ADDR: Map<Addr, ()> = Map::new("BID_RETRACTED_FOR_ADDR");
pub const COMMISSION_BY_ADDR: Map<Addr, Option<Uint128>> = Map::new("COMMISSION_BY_ADDR");
pub const HIGHEST_BIDDER: Item<Option<Addr>> = Item::new("HIGHEST_BIDDER");
pub const BID_CLOSED: Item<bool> = Item::new("BID_CLOSED");
pub const BID_WINNER: Item<Option<Addr>> = Item::new("BID_WINNER");
