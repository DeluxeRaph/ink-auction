#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod auction {
    use core::u128;
    use ink::contract_ref;
    use psp34::{Id, PSP34, PSP34Data, PSP34Event};
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Auction {
        seller: AccountId, // Seller address
        start_time: Timestamp, // This is the start time of the bid.
        end_time: u16, // This will be a timer counts down to the end of the auction
        started: bool, // Checks if auction has started
        ended: bool, // Checks if auction has ended
    }

    pub struct NFT {
        data: PSP34Data,
        id: Id,
    }


    // Keeping track of the bids
    pub struct Auction_bids {
        highestbidder: AccountId, // Highest bidders address
        highest_bid: u128,
        bids: Mapping<AccountId, u32>, // Keeps track of the current bids  
    }

    impl Auction {
        #[ink(constructor)]
        pub fn new(duration: u16) -> Self {
            let caller = Self::env().caller();
            let current_time = Self::env().block_timestamp();
            Self {
                seller: caller,
                start_time: current_time,
                end_time: duration,
                started: true,
                ended: false,
            }
        }

        #[ink(message)]
        pub fn place_bid(&self) {

        }
    }
}
  