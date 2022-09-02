use crate::inline_spl_token::{self, GenericTokenAccount};

solana_sdk::declare_id!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

const ACCOUNTTYPE_ACCOUNT: u8 = 2;

pub struct Account;
impl GenericTokenAccount for Account {
    fn valid_account_data(account_data: &[u8]) -> bool {
        inline_spl_token::Account::valid_account_data(account_data)
            || ACCOUNTTYPE_ACCOUNT
                == *account_data
                    .get(inline_spl_token::Account::get_packed_len())
                    .unwrap_or(&0)
    }
}
