#[cfg(test)]
mod test {
    use crate::test::helper::helpers::{helper_get_owner, helper_instantiate};
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn no_owner_given() {
        let mut deps = mock_dependencies();
        helper_instantiate(&mut deps, None);
        let owner = helper_get_owner(&deps);
        assert_eq!(owner, "contract_instance_owner_addr")
    }

    #[test]
    fn owner_given() {
        let mut deps = mock_dependencies();
        helper_instantiate(&mut deps, Some("new_owner".to_string()));
        let owner = helper_get_owner(&deps);
        assert_eq!(owner, "new_owner")
    }
}
