use crate::error::BidError;
use crate::msg::BidQueryMsg;
use crate::state::{BID_BY_ADDR, BID_CLOSED, BID_WINNER, DENOM, HIGHEST_BIDDER, OWNER};
use cosmwasm_std::{to_binary, Binary, Deps, Env, Uint128};

pub fn _query(deps: Deps, _env: Env, msg: BidQueryMsg) -> Result<Binary, BidError> {
    Ok(match msg {
        BidQueryMsg::GetOwner {} => to_binary(&OWNER.load(deps.storage)?),
        BidQueryMsg::GetDenom {} => to_binary(&DENOM.load(deps.storage)?),
        BidQueryMsg::BidClosed {} => to_binary(&BID_CLOSED.load(deps.storage)?),
        BidQueryMsg::BidForAddress { address } => to_binary(&bid_for_address(deps,address)?),
        BidQueryMsg::HighestBid {} => to_binary(&highest_bid(deps)?),
        BidQueryMsg::HighestBidder {} => to_binary(&HIGHEST_BIDDER.load(deps.storage)?),
        BidQueryMsg::BidWinner {} => to_binary(&BID_WINNER.load(deps.storage)?),
    }?)
}

fn bid_for_address(deps: Deps, address: String) -> Result<Uint128, BidError> {
    let validated_addr = deps.api.addr_validate(address.as_str())?;

    let bid = BID_BY_ADDR.load(deps.storage, validated_addr);

    if let Ok(bid) = bid {
        Ok(bid)
    } else {
        Err(BidError::NoBidPresent)
    }
}

fn highest_bid(deps: Deps) -> Result<Uint128, BidError> {
    let addr = HIGHEST_BIDDER.load(deps.storage)?;

    if let Some(highest_bidder) = addr {
        let bid = BID_BY_ADDR.load(deps.storage, highest_bidder);

        if let Ok(bid) = bid {
            Ok(bid)
        } else {
            Err(BidError::NoBidPresent)
        }
    } else {
        Err(BidError::NoBidPresent)
    }
}