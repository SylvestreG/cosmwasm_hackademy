use crate::msg::BidQueryMsg;
use crate::state::{
    BID_BY_ADDR, BID_CLOSED, BID_WINNER, COMMISSION_BY_ADDR, DENOM, HIGHEST_BIDDER, OWNER,
};
use cosmwasm_std::{to_binary, Binary, Deps, Env, StdError, StdResult, Uint128};
use std::ops::AddAssign;

pub fn _query(deps: Deps, _env: Env, msg: BidQueryMsg) -> StdResult<Binary> {
    match msg {
        BidQueryMsg::GetOwner {} => to_binary(&OWNER.load(deps.storage)?),
        BidQueryMsg::GetDenom {} => to_binary(&DENOM.load(deps.storage)?),
        BidQueryMsg::BidClosed {} => to_binary(&BID_CLOSED.load(deps.storage)?),
        BidQueryMsg::BidForAddress { address } => to_binary(&bid_for_address(deps, address)?),
        BidQueryMsg::HighestBid {} => to_binary(&highest_bid(deps)?),
        BidQueryMsg::HighestBidder {} => to_binary(&HIGHEST_BIDDER.load(deps.storage)?),
        BidQueryMsg::BidWinner {} => to_binary(&BID_WINNER.load(deps.storage)?),
    }
}

fn bid_for_address(deps: Deps, address: String) -> StdResult<Uint128> {
    let validated_addr = deps.api.addr_validate(address.as_str())?;

    let bid = BID_BY_ADDR.load(deps.storage, validated_addr.clone());
    let comm = COMMISSION_BY_ADDR
        .load(deps.storage, validated_addr)?
        .unwrap_or_default();

    if let Ok(mut bid) = bid {
        bid.add_assign(&comm);
        Ok(bid)
    } else {
        Err(StdError::generic_err("no bid present"))
    }
}

fn highest_bid(deps: Deps) -> StdResult<Uint128> {
    let addr = HIGHEST_BIDDER.load(deps.storage)?;

    if let Some(highest_bidder) = addr {
        let bid = BID_BY_ADDR.load(deps.storage, highest_bidder.clone());
        let comm = COMMISSION_BY_ADDR
            .load(deps.storage, highest_bidder)?
            .unwrap_or_default();

        if let Ok(mut bid) = bid {
            bid.add_assign(&comm);
            Ok(bid)
        } else {
            Err(StdError::generic_err("no bid present"))
        }
    } else {
        Err(StdError::generic_err("no bid present"))
    }
}
