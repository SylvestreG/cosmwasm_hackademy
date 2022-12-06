use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum BidError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Owner cannot bid")]
    OwnerCannotBid,

    #[error("Not Enough Coin For Commission")]
    NotEnoughCoinForCommission,

    #[error("Bid too low")]
    BidTooLow,

    #[error("Bid closed")]
    BidClosed,

    #[error("Bid Already Closed")]
    BidAlreadyClosed,

    #[error("Bid Not Closed")]
    BidNotClosed,

    #[error("Only Owner Can Close")]
    OnlyOwnerCanClose,

    #[error("No Bid present")]
    NoBidPresent,

    #[error("Invalid Highest Bidder")]
    InvalidHighestBidder,
}
