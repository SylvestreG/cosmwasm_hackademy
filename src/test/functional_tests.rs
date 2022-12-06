#[cfg(test)]
mod test {
    use crate::test::helper::helpers::{helper_get_bid_closed, helper_get_bid_winner, helper_get_denom, helper_get_highest_bidder, helper_get_owner, helper_instantiate};
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn no_owner_no_denom_given() {
        let mut deps = mock_dependencies();
        helper_instantiate(&mut deps, None, None);
        let owner = helper_get_owner(&deps);
        assert_eq!(owner, "contract_instance_owner_addr");

        let denom = helper_get_denom(&deps);
        assert_eq!(denom, "uatom");

        assert_eq!(None, helper_get_bid_winner(&deps));
        assert_eq!(None, helper_get_highest_bidder(&deps));
        assert!(!helper_get_bid_closed(&deps))
    }

    #[test]
    fn owner_no_denom_given() {
        let mut deps = mock_dependencies();
        helper_instantiate(&mut deps, Some("new_owner".to_string()), None);
        let owner = helper_get_owner(&deps);
        assert_eq!(owner, "new_owner");

        let denom = helper_get_denom(&deps);
        assert_eq!(denom, "uatom");

        assert_eq!(None, helper_get_bid_winner(&deps));
        assert_eq!(None, helper_get_highest_bidder(&deps));
        assert!(!helper_get_bid_closed(&deps))
    }

    #[test]
    fn owner_denom_given() {
        let mut deps = mock_dependencies();
        helper_instantiate(
            &mut deps,
            Some("new_owner".to_string()),
            Some("ujuno".to_string()),
        );
        let owner = helper_get_owner(&deps);
        assert_eq!(owner, "new_owner");

        let denom = helper_get_denom(&deps);
        assert_eq!(denom, "ujuno");

        assert_eq!(None, helper_get_bid_winner(&deps));
        assert_eq!(None, helper_get_highest_bidder(&deps));
        assert!(!helper_get_bid_closed(&deps))
    }
}
