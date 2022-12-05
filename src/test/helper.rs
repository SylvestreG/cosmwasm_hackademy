#[cfg(test)]
pub mod helpers {
    use crate::contract::{instantiate, query};
    use crate::msg::{BidInstantiateMsg, BidQueryMsg};
    use cosmwasm_std::testing::{mock_env, mock_info, MockApi, MockQuerier, MockStorage};
    use cosmwasm_std::{from_binary, Addr, OwnedDeps};

    pub fn helper_instantiate(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>,
        owner: Option<String>,
    ) {
        let msg = BidInstantiateMsg { owner };

        let info = mock_info("contract_instance_owner_addr", &[]);

        // we can just call .unwrap() to assert this was a success
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    #[cfg(test)]
    pub fn helper_get_owner(deps: &OwnedDeps<MockStorage, MockApi, MockQuerier>) -> Addr {
        let res = query(deps.as_ref(), mock_env(), BidQueryMsg::GetOwner {}).unwrap();
        from_binary(&res).unwrap()
    }
}
