#[cfg(test)]
pub mod helpers {
    use crate::contract::{instantiate, query};
    use crate::msg::{BidInstantiateMsg, BidQueryMsg};
    use cosmwasm_std::testing::{mock_env, mock_info, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{from_binary, Addr, OwnedDeps};

    pub fn helper_instantiate(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>,
        owner: Option<String>,
        denom: Option<String>,
    ) {
        let msg = BidInstantiateMsg { owner, denom };

        let info = mock_info("contract_instance_owner_addr", &[]);

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[cfg(test)]
    pub fn helper_get_owner(deps: &OwnedDeps<MockStorage, MockApi, MockQuerier>) -> Addr {
        let res = query(deps.as_ref(), mock_env(), BidQueryMsg::GetOwner {}).unwrap();
        from_binary(&res).unwrap()
    }

    #[cfg(test)]
    pub fn helper_get_denom(deps: &OwnedDeps<MockStorage, MockApi, MockQuerier>) -> Addr {
        let res = query(deps.as_ref(), mock_env(), BidQueryMsg::GetDenom {}).unwrap();
        from_binary(&res).unwrap()
    }

    #[cfg(test)]
    pub fn helper_get_bid_closed(deps: &OwnedDeps<MockStorage, MockApi, MockQuerier>) -> bool {
        let res = query(deps.as_ref(), mock_env(), BidQueryMsg::BidClosed {}).unwrap();
        from_binary(&res).unwrap()
    }

    #[cfg(test)]
    pub fn helper_get_highest_bidder(deps: &OwnedDeps<MockStorage, MockApi, MockQuerier>) -> Option<String> {
        let res = query(deps.as_ref(), mock_env(), BidQueryMsg::HighestBidder {}).unwrap();
        from_binary(&res).unwrap()
    }

    #[cfg(test)]
    pub fn helper_get_bid_winner(deps: &OwnedDeps<MockStorage, MockApi, MockQuerier>) -> Option<String> {
        let res = query(deps.as_ref(), mock_env(), BidQueryMsg::BidWinner {}).unwrap();
        from_binary(&res).unwrap()
    }
}
