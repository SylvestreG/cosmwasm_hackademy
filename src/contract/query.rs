use crate::msg::BidQueryMsg;
use crate::state::OWNER;
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdResult};

pub fn _query(deps: Deps, _env: Env, msg: BidQueryMsg) -> StdResult<Binary> {
    match msg {
        BidQueryMsg::GetOwner {} => to_binary(&OWNER.load(deps.storage)?),
    }
}
