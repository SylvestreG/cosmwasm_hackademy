use crate::msg::BidQueryMsg;
use cosmwasm_std::{Binary, Deps, Env, StdResult};

pub fn _query(_deps: Deps, _env: Env, msg: BidQueryMsg) -> StdResult<Binary> {
    match msg {}
}
