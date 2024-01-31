#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod blind_auction {
    use core::{u128, default};
    use ink::{contract_ref, primitives::AccountId as OtherAccountId};
    use psp34::Id;
    use scale_info::TypeInfo;
    use ink::storage::Mapping;
    use ink::storage::Lazy;
    use ink::prelude::vec::Vec;
    use ink::storage::traits::StorageLayout;
    

    type UserId = AccountId;
    type TokenAddress = AccountId;


    /// Auction status
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

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    /// Error types
    pub enum Error {
        /// Return if bid was placed while auction isn't in active status
        AuctionNotActive,
        /// Return if current bid > current highest bid
        NotOutBidding(Balance, Balance),
        /// Problems with winning_data observed
        WinningDataCorrupted,
    }

    // Auction struct is storing all data types for an auction
    #[ink(storage)]
    pub struct Auction {
        /// Seller address
        seller: UserId, 
        /// This is the starting blocknumber for the auction
        start_block: BlockNumber,
        /// This will be a timer counts down to the end of the auction
        auction_duration: BlockNumber,
        end_block: BlockNumber,
        /// Finalization of auction  
        finalized: bool,
        /// Bidders balances storage.  
        /// Current user's balance = her top bid
        balances: Mapping<AccountId, Balance>,
        /// Address of the NFT that is being bid on
        nft_contract: TokenAddress,
        /// The Id or number of the NFT
        nft_id: u32,
        /// The Seller's minimum starting bid price
        min_bid: Balance,
        /// Keeps track of the current bids
        /// Maps bidders address to their bid 
        bids: Mapping<UserId, Balance>,
        highest_bid: Balance,
        /// Keeps track of the current hightest bid
        /// Option is waiting for the address of the highest bidder
        highest_bidder: Option<UserId>,
        /// Maps the winning bid to the winning address  
        winner: Option<(UserId, Balance)>,
        /// AuctionData = storage of winners per block
        /// it's a vector of optional (AccountId, Balance) tuples representing winner in block along with bid
        /// 0 indexed value is winner for OpeningPeriod
        /// i indexed value is winner for
        auction_data: Vec<Option<(UserId, Balance)>>,
    }

    /// Event emitted when a bid is accepted.
    #[ink(event)]
    pub struct Bid {
        #[ink(topic)]
        from: UserId,

        bid: Balance,
    }

    impl Auction {
        #[ink(constructor)]
        pub fn new(
            nft_contract: TokenAddress,
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

            /// Auction data tracks the total bids in block
            let mut auction_data = Vec::<Option<(UserId, Balance)>>::new();
            (0..to_block + 1).for_each(|_| auction_data.insert(0,None));


            Self {
                seller: caller,
                nft_contract,
                nft_id,
                min_bid,
                start_block: current_block,
                end_block: 0,
                auction_duration: 0, // There always needs to have a value
                balances: Mapping::new(),
                bids: Mapping::new(),
                highest_bid: 0,
                highest_bidder: None,
                winner: None,
                finalized: false,
                auction_data,
            }
        }

        /// Checks current status of the auction
        fn status(&self, block: BlockNumber) -> Status {
            /// Check for bug second start_block
            let opening_period_last_block = self.start_block + self.start_block - 1;
            let ending_period_last_block = opening_period_last_block + self.end_block;

            match block {
                current_block if current_block < self.start_block => Status::NotStarted,
                current_block if current_block <= opening_period_last_block => Status::Started,
                // number of slot = number of block inside ending period
                current_block if current_block == ending_period_last_block => Status::BlockSnapshot(block - opening_period_last_block),
                _=> Status::Ended
            }
        }

    #[ink(message, payable)]        
    pub fn place_bid(
        &mut self,
        bidder: UserId,
        bid: Balance,
        current_block: BlockNumber,
     ) -> Result<(), Error> {
        let auction_status = self.status(current_block);
        /// 
        let opening_period_start_block = 
        match auction_status {
            Status::Started => 0,
            Status::BlockSnapshot(block_number) => block_number,
            _ => return Err(Error::AuctionNotActive),
        };

        // do not accept bids less than the current highest bid
        if let Some(highest_bidder) = self.highest_bidder {
            let winning_amount = self.balances.get(&highest_bidder).unwrap_or(0);
            if bid < winning_amount {
                return Err(Error::NotOutBidding(bid, winning_amount));
            }
        }

            // Bid is placed
            self.balances.insert(bidder, &bid);
            self.highest_bidder = Some(bidder);
            // update highest bid
            // match self.auction_data.insert(opening_period_start_block as usize, Some((bidder, bid))) {
            //     Err(ink_storage::collections::vec::IndexOutOfBounds) => {
            //         Err(Error::WinningDataCorrupted)
            //     }
            //     Ok(_) => {
            //         self.env().emit_event(Bid {
            //             from: bidder,
            //             bid: bid,
            //         });
            //         Ok(())
            //     }
            // }
            Ok(())
    }
}}
  