#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::new_without_default))]
#[ink::contract]

mod environment_helpers {
    // estas funciones siempre estan disponibles en ink
    #[ink(storage)]
    pub struct EnvironmentHelpers {}

    impl EnvironmentHelpers {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get_address(&self) -> AccountId {
            self.env().account_id()
        }

        #[ink(message)]
        pub fn get_native_balance(&self) -> Balance {
            self.env().balance()
        }

        #[ink(message)]
        pub fn get_current_block_number(&self) -> BlockNumber {
            self.env().block_number()
        }

        #[ink(message)]
        pub fn get_current_block_timestamp(&self) -> Timestamp {
            self.env().block_timestamp()
        }

        #[ink(message)]
        pub fn get_caller(&self) -> AccountId {
            self.env().caller()
        }

        #[ink(message)]
        pub fn is_contract(&self, account_id: AccountId) -> bool {
            self.env().is_contract(&account_id)
        }
    }
}