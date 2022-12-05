use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const OWNER: Item<Addr> = Item::new("OWNER");
// pub const BID_BY_ADDR: Map<Addr, Uint128> = Map::new("BID_BY_ADDR");
