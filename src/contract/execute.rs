use crate::error::BidError;
use crate::msg::BidExecuteMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

pub fn _execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: BidExecuteMsg,
) -> Result<Response, BidError> {
    todo!()
}
