#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod magink {
    use crate::ensure;swan
    use ink::storage::Mapping;
    use openbrush::traits::Storage;
    use wizard::wizard;
    ownable::Internal::_init_with_owner(&mut _instance, Self::env().caller());
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        TooEarlyToClaim,
        UserNotFound,
    }

    #[ink(storage)]
    pub struct Magink {
        user: Mapping<AccountId, Profile>,
    }
    #[derive(
        Debug, PartialEq, Eq, PartialOrd, Ord, Clone, scale::Encode, scale::Decode,
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Profile {
        // duration in blocks until next claim
        claim_era: u8,
        // block number of last claim
        start_block: u32,
        // number of badges claimed
        badges_claimed: u8,
    }

    impl Magink {
        /// Creates a new Magink smart contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                user: Mapping::new(),
            }
        }

        /// (Re)Start the Magink the claiming era for the caller.
        #[ink(message)]
        pub fn start(&mut self, era: u8) {
            let profile = Profile {
                claim_era: era,
                start_block: self.env().block_number(),
                badges_claimed: 0,
            };
            self.user.insert(self.env().caller(), &profile);
        }

        /// Claim the badge after the era.
        #[ink(message)]
        pub fn claim(&mut self) -> Result<(), Error> {
            ensure!(self.get_remaining() == 0, Error::TooEarlyToClaim);

            // update profile
            let mut profile = self.get_profile().ok_or(Error::UserNotFound).unwrap();
            profile.badges_claimed += 1;
            profile.start_block = self.env().block_number();
            self.user.insert(self.env().caller(), &profile);
            Ok(())
        }

        /// Returns the remaining blocks in the era.
        #[ink(message)]
        pub fn get_remaining(&self) -> u8 {

            let current_block = self.env().block_number();
            let caller = self.env().caller();
            self.user.get(&caller).map_or(0, |profile| {
                if current_block - profile.start_block >= profile.claim_era as u32 {
                    return 0;
                }
                profile.claim_era - (current_block - profile.start_block) as u8
            })
        }

        /// Returns the remaining blocks in the era for the given account.
        #[ink(message)]
        pub fn get_remaining_for(&self, account: AccountId) -> u8 {

            let current_block = self.env().block_number();
            self.user.get(&account).map_or(0, |profile| {
                if current_block - profile.start_block >= profile.claim_era as u32 {
                    return 0;
                }
                profile.claim_era - (current_block - profile.start_block) as u8
            })
        }

        /// Returns the profile of the given account.
        #[ink(message)]
        pub fn get_account_profile(&self, account: AccountId) -> Option<Profile> {
            self.user.get(&account)
        }
        
        /// Returns the profile of the caller.
        #[ink(message)]
        pub fn get_profile(&self) -> Option<Profile> {
            let caller = self.env().caller();
            self.user.get(&caller)
        }

        /// Returns the badge of the caller.
        #[ink(message)]
        pub fn get_badges(&self) -> u8 {
            self.get_profile().map_or(0, |profile| profile.badges_claimed)
        }

        /// Returns the badge count of the given account.
        #[ink(message)]
        pub fn get_badges_for(&self, account: AccountId) -> u8 {
            self.get_account_profile(account).map_or(0, |profile| profile.badges_claimed)
        }

        #[ink(message)]
        pub fn mint_wizard(&mut self, to: AccountId, token_id: u64) -> Result<(), ink_env::Error> {
            // Ensure the user has collected all badges (you can use your criteria here)
            ensure!(self.get_badges() >= REQUIRED_BADGES, Error::InsufficientBadges);
    
            // Get a reference to the wizard contract
            let wizard_contract = self.env().account_id::<wizard::Wizard>();
    
            // Call the mint function on the wizard contract
            wizard_contract.mint(to, token_id)?;
    
            Ok(())

    }

    #![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::string::String;

#[ink::contract]
mod wizard_contract {
    use ink_storage::collections::HashMap as StorageHashMap;
    use ink_prelude::string::String;

    #[ink(storage)]
    pub struct WizardContract {
        owner: AccountId,
        wizards: StorageHashMap<u32, AccountId>,
    }

    impl WizardContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                wizards: StorageHashMap::new(),
            }
        }

        // Mint a Wizard-NFT
        #[ink(message)]
        pub fn mint_wizard(&mut self, recipient: AccountId, token_id: u32) -> Result<(), String> {
            self.ensure_owner()?;
            if self.wizards.contains_key(&token_id) {
                return Err(String::from("Token already minted"));
            }
            self.wizards.insert(token_id, recipient);
            
            // Emit the Transfer event
            self.env().emit_event(Transfer {
                from: AccountId::from([0x00; 32]), // Replace with appropriate value
                to: recipient,
                token_id,
            });

            Ok(())
        }

        // Transfer a Wizard-NFT
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, token_id: u32) -> Result<(), String> {
            self.ensure_owner()?;
            let owner = self.wizards.get(&token_id)
                .ok_or_else(|| String::from("Token not minted"))?;
            self.wizards.insert(token_id, to);

            // Emit the Transfer event
            self.env().emit_event(Transfer {
                from: owner,
                to,
                token_id,
            });

            Ok(())
        }

        // Helper function to ensure the sender is the contract owner
        fn ensure_owner(&self) -> Result<(), String> {
            let caller = self.env().caller();
            if caller != self.owner {
                return Err(String::from("Only contract owner can perform this action"));
            }
            Ok(())
        }
        
        // Define the Transfer event for NFT minting and transferring
        #[ink(event)]
        pub struct Transfer {
            #[ink(topic)]
            from: AccountId,
            #[ink(topic)]
            to: AccountId,
            #[ink(topic)]
            token_id: u32,
        }
    }
}


    #![cfg(test)]

use ink_lang as ink;
use ink_env::Environment;
use ink_env::test::*;
use ink_prelude::string::String;

#[ink::test]
fn e2e_tests() {
    // Arrange
    let accounts = default_accounts::<Environment>();
    let mut magink = Magink::new();
    let mut wizard = WizardContract::new();

    // Deploy the contracts
    let magink_instance = deploy_contract::<Magink>(
        ink_env::call::CreateBuilder::default()
            .caller(accounts.alice)
            .gas_limit(1_000_000)
            .endowment(1_000_000)
            .code(magink.code_hash().to_vec()),
    );

    let wizard_instance = deploy_contract::<WizardContract>(
        ink_env::call::CreateBuilder::default()
            .caller(accounts.bob)
            .gas_limit(1_000_000)
            .endowment(1_000_000)
            .code(wizard.code_hash().to_vec()),
    );


    execute_contract_call(
        &mut magink,
        &magink_instance,
        accounts.alice,
        magink_instance.env().block_number() + 1,
        10,
        "start",
    );

 
    advance_block();
    execute_contract_call(
        &mut magink,
        &magink_instance,
        accounts.alice,
        magink_instance.env().block_number() + 1,
        0,
        "claim",
    );

    // User mints a Wizard-NFT
    execute_contract_call(
        &mut magink,
        &magink_instance,
        accounts.alice, // Caller
        0,              // Gas limit
        0,           
        "mint_wizard",  // Call the mint_wizard function
    );

    assert_eq!(magink.get_badges(), 1);

    let owner_of_wizard_nft = wizard_instance.call(
        "owner_of",
        vec![
wizard_contract.mint_wizard(account_id, 1)?;
wizard_contract.mint_wizard(account_id, 2)?;

        ],
    );
    let total_supply = wizard_instance.call("total_supply", vec![]);

}
    mod tests {
        use super::*;

        #[ink::test]
        fn start_works() {
            let mut magink = Magink::new();
            println!("get {:?}", magink.get_remaining());
            magink.start(10);
            assert_eq!(10, magink.get_remaining());
            advance_block();
            assert_eq!(9, magink.get_remaining());
        }

        #[ink::test]
        fn claim_works() {
            const ERA: u32 = 10;
            let accounts = default_accounts();
            let mut magink = Magink::new();
            magink.start(ERA as u8);
            advance_n_blocks(ERA - 1);
            assert_eq!(1, magink.get_remaining());

            // claim fails, too early
            assert_eq!(Err(Error::TooEarlyToClaim), magink.claim());
            
            // claim succeeds
            advance_block();
            assert_eq!(Ok(()), magink.claim());
            assert_eq!(1, magink.get_badges());
            assert_eq!(1, magink.get_badges_for(accounts.alice));
            assert_eq!(1, magink.get_badges());
            assert_eq!(10, magink.get_remaining());
            
            // claim fails, too early
            assert_eq!(Err(Error::TooEarlyToClaim), magink.claim());
            advance_block();
            assert_eq!(9, magink.get_remaining());
            assert_eq!(Err(Error::TooEarlyToClaim), magink.claim());
        }

        fn default_accounts() -> ink::env::test::DefaultAccounts<ink::env::DefaultEnvironment> {
            ink::env::test::default_accounts::<Environment>()
        }

        // fn set_sender(sender: AccountId) {
        //     ink::env::test::set_caller::<Environment>(sender);
        // }
        fn advance_n_blocks(n: u32) {
            for _ in 0..n {
                advance_block();
            }
        }
        fn advance_block() {
            ink::env::test::advance_block::<ink::env::DefaultEnvironment>();
        }
    }
}

/// Evaluate `$x:expr` and if not true return `Err($y:expr)`.
///
/// Used as `ensure!(expression_to_ensure, expression_to_return_on_false)`.
#[macro_export]
macro_rules! ensure {
    ( $x:expr, $y:expr $(,)? ) => {{
        if !$x {
            return Err($y.into());
        }
    }};
}