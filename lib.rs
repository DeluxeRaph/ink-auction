#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod auction {
    use core::{u128, default};
    use ink::{contract_ref, primitives::AccountId as OtherAccountId};
    use psp34::Id;
    use scale_info::TypeInfo;
    use ink::storage::Mapping;
    use ink::storage::traits::StorageLayout;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Status {
        /// The auction has not started.
        NotStarted,
        /// Auction has started
        Started,
        /// A shapshot was taken of the highest bid on this block
        BlockSnapshot(BlockNumber),
        /// Auction has ended
        Ended,
    }

    // Auction struct is storing all data types for an auction
    #[ink(storage)]
    // #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode)]
    // #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, StorageLayout))]
    pub struct Auction {
        /// Seller address
        seller: AccountId, 
        /// This is the starting blocknumber for the auction
        start_block: BlockNumber,
        to_block: BlockNumber
        /// Address of the NFT that is being bid on
        nft_contract: AccountId,
        /// The Id or number of the NFT
        nft_id: u32,
        /// The Seller's minimum starting bid price
        min_bid: Balance,
        /// This will be a timer counts down to the end of the auction
        auction_duration: BlockNumber,
        /// Keeps track of the current bids
        /// Maps bidders address to their bid 
        bids: Mapping<AccountId, Balance>,
        highest_bid: Balance,
        /// Keeps track of the current hightest bid
        /// Option is waiting for the address of the highest bidder
        highest_bidder: Option<AccountId>,
        /// Maps the winning bid to the winning address  
        winner: Option<AccountId>,
        /// Checks if auction has started
        started: bool,
        /// Checks if auction has ended 
        ended: bool, 
    }

    impl Auction {
        #[ink(constructor)]
        pub fn new(
            nft_contract: AccountId,
            nft_id: u32,
            min_bid: Balance,
            start_block: Option<BlockNumber>,
            to_block: BlockNumber,

        ) -> Self {
            let caller = Self::env().caller();
            let current_block = Self::env().block_number();
            /// Auction starts on the following block
            let start_auction = start_block.unwrap_or(current_block + 1);

            assert!(
                start_auction > current_block,
                "Auction is not allowed to be scheduled on future blocks"
            );
            Self {
                seller: caller,
                nft_contract,
                nft_id,
                min_bid,
                start_block: current_block,
                to_block: 0,
                auction_duration,
                bids: Mapping::new(),
                highest_bid: 0,
                highest_bidder: None,
                winner: None,
                started: true,
                ended: false,
            }
        }

        #[ink(message, payable)]        
        pub fn place_bid(&self) {
            &mut self,
            bidder: AccountId,
            bid: Balance,
            current_time: Timestamp,
        } -> Result<(), Error> {

        }
    }
}
  