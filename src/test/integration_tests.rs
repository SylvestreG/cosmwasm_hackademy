#[cfg(test)]
mod integration {
    use crate::contract::{execute, instantiate, query};
    use crate::error::BidError;
    use crate::error::BidError::{BidWinner, NoBidPresent};
    use crate::msg::{BidExecuteMsg, BidInstantiateMsg, BidQueryMsg};
    use cosmwasm_std::{coin, coins, Addr, Coin, Empty, StdResult, Uint128};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    pub struct BidContract(Addr);

    impl BidContract {
        #[track_caller]
        pub fn instantiate(
            app: &mut App,
            code_id: u64,
            sender: &Addr,
            label: String,
            owner: Option<String>,
            denom: Option<String>,
        ) -> StdResult<BidContract> {
            app.instantiate_contract(
                code_id,
                sender.clone(),
                &BidInstantiateMsg { owner, denom },
                &[],
                label,
                None,
            )
            .map_err(|err| err.downcast().unwrap())
            .map(BidContract)
        }

        #[track_caller]
        pub fn denom(&self, app: &App) -> StdResult<String> {
            app.wrap()
                .query_wasm_smart(self.0.clone(), &BidQueryMsg::GetDenom {})
        }

        #[track_caller]
        pub fn owner(&self, app: &App) -> StdResult<String> {
            app.wrap()
                .query_wasm_smart(self.0.clone(), &BidQueryMsg::GetOwner {})
        }

        #[track_caller]
        pub fn closed(&self, app: &App) -> StdResult<bool> {
            app.wrap()
                .query_wasm_smart(self.0.clone(), &BidQueryMsg::BidClosed {})
        }

        #[track_caller]
        pub fn winner(&self, app: &App) -> StdResult<Option<String>> {
            app.wrap()
                .query_wasm_smart(self.0.clone(), &BidQueryMsg::BidWinner {})
        }

        #[track_caller]
        pub fn highest_bid(&self, app: &App) -> StdResult<Uint128> {
            app.wrap()
                .query_wasm_smart(self.0.clone(), &BidQueryMsg::HighestBid {})
        }

        #[track_caller]
        pub fn highest_bidder(&self, app: &App) -> StdResult<Option<Addr>> {
            app.wrap()
                .query_wasm_smart(self.0.clone(), &BidQueryMsg::HighestBidder {})
        }

        #[track_caller]
        pub fn bid(&self, app: &mut App, sender: &Addr, funds: &[Coin]) -> Result<(), BidError> {
            app.execute_contract(
                sender.clone(),
                self.0.clone(),
                &BidExecuteMsg::Bid {},
                funds,
            )
            .map_err(|err| err.downcast::<BidError>().unwrap())?;
            Ok(())
        }

        #[track_caller]
        pub fn close(&self, app: &mut App, sender: &Addr, funds: &[Coin]) -> Result<(), BidError> {
            app.execute_contract(
                sender.clone(),
                self.0.clone(),
                &BidExecuteMsg::Close {},
                funds,
            )
            .map_err(|err| err.downcast::<BidError>().unwrap())?;
            Ok(())
        }

        #[track_caller]
        pub fn retract(
            &self,
            app: &mut App,
            sender: &Addr,
            funds: &[Coin],
            receiver: Option<String>,
        ) -> Result<(), BidError> {
            app.execute_contract(
                sender.clone(),
                self.0.clone(),
                &BidExecuteMsg::Retract { receiver },
                funds,
            )
            .map_err(|err| err.downcast::<BidError>().unwrap())?;
            Ok(())
        }
    }

    fn bid_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);

        Box::new(contract)
    }

    #[test]
    fn no_owner_no_denom_given() -> StdResult<()> {
        let mut app = App::default();

        let contract_id = app.store_code(bid_contract());
        let bid = BidContract::instantiate(
            &mut app,
            contract_id,
            &Addr::unchecked("owner"),
            "label".to_string(),
            None,
            None,
        )?;

        assert_eq!(bid.denom(&app)?, "uatom");
        assert_eq!(bid.owner(&app)?, "owner");
        assert!(!bid.closed(&app)?);
        assert_eq!(bid.winner(&app)?, None);
        assert_eq!(bid.highest_bidder(&app)?, None);
        assert!(bid.highest_bid(&app).is_err());

        Ok(())
    }

    #[test]
    fn owner_no_denom_given() -> StdResult<()> {
        let mut app = App::default();

        let contract_id = app.store_code(bid_contract());
        let bid = BidContract::instantiate(
            &mut app,
            contract_id,
            &Addr::unchecked("owner"),
            "label".to_string(),
            Some("test".to_string()),
            None,
        )?;

        assert_eq!(bid.denom(&app)?, "uatom");
        assert_eq!(bid.owner(&app)?, "test");
        assert!(!bid.closed(&app)?);
        assert_eq!(bid.winner(&app)?, None);
        assert_eq!(bid.highest_bidder(&app)?, None);
        assert!(bid.highest_bid(&app).is_err());

        Ok(())
    }

    #[test]
    fn owner_denom_given() -> StdResult<()> {
        let mut app = App::default();

        let contract_id = app.store_code(bid_contract());
        let bid = BidContract::instantiate(
            &mut app,
            contract_id,
            &Addr::unchecked("owner"),
            "label".to_string(),
            Some("test".to_string()),
            Some("ujuno".to_string()),
        )?;

        assert_eq!(bid.denom(&app)?, "ujuno");
        assert_eq!(bid.owner(&app)?, "test");
        assert!(!bid.closed(&app)?);
        assert_eq!(bid.winner(&app)?, None);
        assert_eq!(bid.highest_bidder(&app)?, None);
        assert!(bid.highest_bid(&app).is_err());

        Ok(())
    }

    #[test]
    fn close_with_errors() -> StdResult<()> {
        let user1 = Addr::unchecked("user1");

        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &user1, coins(10_000_000u128, "ujuno"))
                .unwrap();
        });

        let contract_id = app.store_code(bid_contract());
        let bid = BidContract::instantiate(
            &mut app,
            contract_id,
            &Addr::unchecked("owner"),
            "label".to_string(),
            Some("test".to_string()),
            Some("ujuno".to_string()),
        )?;

        assert_eq!(
            bid.close(&mut app, &Addr::unchecked("owner"), &[]),
            Err(BidError::OnlyOwnerCanClose)
        );
        assert_eq!(
            bid.close(&mut app, &Addr::unchecked("test"), &[]),
            Err(BidError::NoBidPresent)
        );

        assert_eq!(
            bid.bid(
                &mut app,
                &Addr::unchecked("user1"),
                &[Coin {
                    denom: "ujuno".to_string(),
                    amount: Uint128::from(5_000_000u128)
                }]
            ),
            Ok(())
        );

        assert_eq!(bid.close(&mut app, &Addr::unchecked("test"), &[]), Ok(()));

        assert_eq!(
            bid.close(&mut app, &Addr::unchecked("test"), &[]),
            Err(BidError::BidAlreadyClosed)
        );

        Ok(())
    }

    #[test]
    fn scenario() -> StdResult<()> {
        let owner = Addr::unchecked("owner");
        let alex = Addr::unchecked("alex");
        let ann = Addr::unchecked("ann");

        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &alex, coins(100_000_000u128, "uatom"))
                .unwrap();

            router
                .bank
                .init_balance(storage, &ann, coins(100_000_000u128, "uatom"))
                .unwrap();
        });

        let contract_id = app.store_code(bid_contract());
        let bid = BidContract::instantiate(
            &mut app,
            contract_id,
            &owner,
            "label".to_string(),
            None,
            None,
        )?;

        assert_eq!(
            bid.bid(&mut app, &alex, &coins(15_000_000, "uatom".to_string())),
            Ok(()),
        );

        assert_eq!(bid.highest_bidder(&app)?, Some(alex.clone()));
        assert_eq!(bid.highest_bid(&app)?, Uint128::new(15_000_000u128));

        assert_eq!(
            bid.bid(&mut app, &ann, &coins(17_000_000, "uatom".to_string())),
            Ok(()),
        );
        assert_eq!(bid.highest_bidder(&app)?, Some(ann.clone()));
        assert_eq!(bid.highest_bid(&app)?, Uint128::new(17_000_000u128));

        assert_eq!(
            bid.bid(&mut app, &ann, &coins(2_000_000, "uatom".to_string())),
            Ok(()),
        );
        assert_eq!(bid.highest_bidder(&app)?, Some(ann.clone()));
        assert_eq!(bid.highest_bid(&app)?, Uint128::new(19_000_000u128));

        assert_eq!(
            bid.bid(&mut app, &alex, &coins(1_000_000, "uatom".to_string())),
            Err(BidError::BidTooLow),
        );
        assert_eq!(bid.highest_bidder(&app)?, Some(ann.clone()));
        assert_eq!(bid.highest_bid(&app)?, Uint128::new(19_000_000u128));

        assert_eq!(
            bid.bid(&mut app, &alex, &coins(5_000_000, "uatom".to_string())),
            Ok(()),
        );
        assert_eq!(bid.highest_bidder(&app)?, Some(alex.clone()));
        assert_eq!(bid.highest_bid(&app)?, Uint128::new(20_000_000u128));

        assert_eq!(bid.close(&mut app, &owner, &[]), Ok(()),);
        assert_eq!(bid.highest_bidder(&app)?, Some(alex.clone()));
        assert_eq!(bid.highest_bid(&app)?, Uint128::new(20_000_000u128));

        // 20 from alex
        // 1 from ann fees
        assert_eq!(
            app.wrap().query_balance(owner.clone(), "uatom")?,
            coin(21_000_000u128, "uatom")
        );

        assert_eq!(
            app.wrap().query_balance(alex.clone(), "uatom")?,
            coin(80_000_000u128, "uatom")
        );
        assert_eq!(
            app.wrap().query_balance(ann.clone(), "uatom")?,
            coin(81_000_000u128, "uatom")
        );

        assert_eq!(bid.retract(&mut app, &owner, &[], None), Err(NoBidPresent));
        assert_eq!(
            bid.retract(&mut app, &owner, &[], Some(alex.to_string())),
            Err(BidWinner)
        );
        assert_eq!(
            bid.retract(&mut app, &owner, &[], Some(ann.to_string())),
            Ok(())
        );
        assert_eq!(
            bid.retract(&mut app, &owner, &[], Some(ann.to_string())),
            Err(BidError::RetractAlreadyDone)
        );

        assert_eq!(
            app.wrap().query_balance(alex.clone(), "uatom")?,
            coin(80_000_000u128, "uatom")
        );
        assert_eq!(
            app.wrap().query_balance(ann.clone(), "uatom")?,
            coin(99_000_000u128, "uatom")
        );

        Ok(())
    }
}
