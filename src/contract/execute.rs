use crate::error::BidError;
use crate::msg::BidExecuteMsg;
use crate::state::{
    BID_BY_ADDR, BID_CLOSED, BID_RETRACTED_FOR_ADDR, BID_WINNER, COMMISSION_BY_ADDR, DENOM,
    HIGHEST_BIDDER, OWNER,
};
use cosmwasm_std::{
    has_coins, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128,
};
use std::ops::{AddAssign, SubAssign};

// constant cut of 0.5 token by bid
const CONTRACT_COMMISSION: u128 = 500_000u128;

pub fn _execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: BidExecuteMsg,
) -> Result<Response, BidError> {
    match msg {
        BidExecuteMsg::Bid {} => bid(deps, info),
        BidExecuteMsg::Close {} => close(deps, info),
        BidExecuteMsg::Retract { receiver } => retract(deps, info, receiver),
    }
}

fn bid(deps: DepsMut, info: MessageInfo) -> Result<Response, BidError> {
    if BID_CLOSED.load(deps.storage)? {
        return Err(BidError::BidClosed);
    }

    if info.sender == OWNER.load(deps.storage)? {
        return Err(BidError::OwnerCannotBid);
    }

    // Check if there is enough coin on for comission plus at least
    let accepted_denom = DENOM.load(deps.storage)?;
    if !has_coins(
        &info.funds,
        &Coin {
            denom: accepted_denom.clone(),
            amount: Uint128::from(CONTRACT_COMMISSION),
        },
    ) {
        return Err(BidError::NotEnoughCoinForCommission);
    }

    // Get from state the accepted denom.
    let highest_bidder = HIGHEST_BIDDER.load(deps.storage)?;

    let get_bid = |addr| -> Uint128 {
        if let Ok(bid) = BID_BY_ADDR.load(deps.storage, addr) {
            bid
        } else {
            Uint128::from(0u128)
        }
    };

    // get last bid.
    let highest_bid = if let Some(bidder) = highest_bidder {
        get_bid(bidder)
    } else {
        Uint128::from(0u128)
    };
    let current_bid = get_bid(info.sender.clone());

    // get the amount of token to send
    let mut new_bid = info
        .funds
        .iter()
        .filter(|c| c.denom == accepted_denom)
        .map(|m| m.amount)
        .sum::<Uint128>();
    new_bid.sub_assign(Uint128::from(CONTRACT_COMMISSION));
    new_bid.add_assign(current_bid);

    let mut commission = COMMISSION_BY_ADDR
        .load(deps.storage, info.sender.clone())
        .unwrap_or_default();

    if let Some(comission) = &mut commission {
        comission.add_assign(Uint128::from(CONTRACT_COMMISSION));
    } else {
        commission = Some(Uint128::from(CONTRACT_COMMISSION));
    }

    if new_bid <= highest_bid {
        return Err(BidError::BidTooLow);
    }

    let commission_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: OWNER.load(deps.storage)?.to_string(),
        amount: vec![Coin {
            denom: accepted_denom,
            amount: Uint128::from(CONTRACT_COMMISSION),
        }],
    });

    COMMISSION_BY_ADDR.save(deps.storage, info.sender.clone(), &commission)?;
    HIGHEST_BIDDER.save(deps.storage, &Some(info.sender.clone()))?;
    BID_BY_ADDR.save(deps.storage, info.sender, &new_bid)?;

    Ok(Response::new()
        .add_message(commission_msg)
        .add_attribute("sent amount", new_bid.to_string())
        .add_attribute("commission", CONTRACT_COMMISSION.to_string())
        .add_attribute("method", "bid"))
}

fn close(deps: DepsMut, info: MessageInfo) -> Result<Response, BidError> {
    if BID_CLOSED.load(deps.storage)? {
        return Err(BidError::BidAlreadyClosed);
    }

    if OWNER.load(deps.storage)? != info.sender {
        return Err(BidError::OnlyOwnerCanClose);
    }

    let highest_bidder = HIGHEST_BIDDER.load(deps.storage)?;
    if highest_bidder.is_none() {
        return Err(BidError::NoBidPresent);
    }
    let highest_bidder = highest_bidder.unwrap();

    let bid_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: OWNER.load(deps.storage)?.to_string(),
        amount: vec![Coin {
            denom: DENOM.load(deps.storage)?,
            amount: BID_BY_ADDR.load(deps.storage, highest_bidder.clone())?,
        }],
    });

    BID_CLOSED.save(deps.storage, &true)?;
    BID_WINNER.save(deps.storage, &Some(highest_bidder.clone()))?;

    Ok(Response::new()
        .add_message(bid_msg)
        .add_attribute("winner", highest_bidder.to_string())
        .add_attribute("method", "close"))
}

fn retract(
    deps: DepsMut,
    info: MessageInfo,
    receiver: Option<String>,
) -> Result<Response, BidError> {
    if !BID_CLOSED.load(deps.storage)? {
        return Err(BidError::BidNotClosed);
    }

    let recipient = if let Some(addr) = receiver {
        deps.api.addr_validate(&addr)?
    } else {
        info.sender
    };

    if BID_WINNER.load(deps.storage)?.unwrap() == recipient {
        return Err(BidError::BidWinner);
    }

    if BID_RETRACTED_FOR_ADDR.has(deps.storage, recipient.clone()) {
        return Err(BidError::RetractAlreadyDone);
    } else {
        BID_RETRACTED_FOR_ADDR.save(deps.storage, recipient.clone(), &())?;
    }

    let bid = BID_BY_ADDR
        .load(deps.storage, recipient.clone())
        .map_err(|_| BidError::NoBidPresent)?;

    let retract_msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: recipient.to_string(),
        amount: vec![Coin {
            denom: DENOM.load(deps.storage)?,
            amount: bid,
        }],
    });

    Ok(Response::new()
        .add_message(retract_msg)
        .add_attribute("amount", bid.to_string())
        .add_attribute("recipient", recipient.to_string())
        .add_attribute("method", "retract"))
}
