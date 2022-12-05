use crate::error::BidError;
use crate::msg::BidInstantiateMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn _instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: BidInstantiateMsg,
) -> Result<Response, BidError> {
    Ok(Response::new().add_attribute("method", "instantiate"))
}
