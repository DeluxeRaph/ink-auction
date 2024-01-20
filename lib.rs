#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod auction {
    use core::u128;
    use ink::{contract_ref, primitives::AccountId as OtherAccountId};
    use psp34::{ Id, PSP34Data, PSP34Event };
    use ink::storage::Mapping;

    // Auction struct is storing all data types for an auction
    #[ink(storage)]
    pub struct Auction {
        /// Seller address
        seller: AccountId, 
        /// This is the start time of the bid
        start_time: Timestamp,
        /// Address of the NFT that is being bid on
        nft_contract: AccountId,
        /// The Id or number of the NFT
        nft_id: Id,
        /// The Seller's minimum starting bid price
        min_bid: Balance,
        /// This will be a timer counts down to the end of the auction
        auction_duration: Timestamp,
        /// Keeps track of the current bids
        /// Maps bidders address to their bid 
        bids: Mapping<AccountId, Balance>,
        /// Keeps track of the current hightest bid
        /// Option is waiting for the address of the highest bidder
        highest_bid_address: Option<AccountId>,
        /// Maps the winning bid to the winning address  
        winner: Mapping<AccountId, Balance>,
        /// Checks if auction has started
        started: bool,
        /// Checks if auction has ended 
        ended: bool, 
    }

    // Keeping track of the bids
    pub struct Auction_bids {
        highestbidder: AccountId, // Highest bidders address
    }

    impl Auction {
        #[ink(constructor)]
        pub fn new(
            nft_contract: AccountId,
            nft_id: Id,
            min_bid: Balance,
            auction_duration: Timestamp,

        ) -> Self {
            let caller = Self::env().caller();
            let current_time = Self::env().block_timestamp();
            Self {
                seller: caller,
                start_time: current_time,
                auction_duration,
                started: true,
                ended: false,
            }
        }

        #[ink(message)]
        pub fn place_bid(&self) {
            
        }
    }
}
  