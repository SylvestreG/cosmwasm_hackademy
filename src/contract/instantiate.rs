use crate::error::BidError;
use crate::msg::BidInstantiateMsg;
use crate::state::{BID_CLOSED, BID_WINNER, DENOM, HIGHEST_BIDDER, OWNER};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn _instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: BidInstantiateMsg,
) -> Result<Response, BidError> {
    let owner = if let Some(owner) = msg.owner {
        deps.api.addr_validate(owner.as_str())?
    } else {
        info.sender
    };

    let denom = msg.denom.unwrap_or("uatom".to_string());

    OWNER.save(deps.storage, &owner)?;
    DENOM.save(deps.storage, &denom)?;
    HIGHEST_BIDDER.save(deps.storage, &None)?;
    BID_CLOSED.save(deps.storage, &false)?;
    BID_WINNER.save(deps.storage, &None)?;

    Ok(Response::new()
        .add_attribute("owner", owner)
        .add_attribute("method", "instantiate"))
}
