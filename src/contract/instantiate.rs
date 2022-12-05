use crate::error::BidError;
use crate::msg::BidInstantiateMsg;
use crate::state::OWNER;
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

    OWNER.save(deps.storage, &owner)?;

    Ok(Response::new()
        .add_attribute("owner", owner)
        .add_attribute("method", "instantiate"))
}
