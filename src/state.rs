use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    program_error::ProgramError,
};

pub const SEED_BLEPAY: &str = "blepay";
pub const SEED_CREATE_DOMAIN: &str = "create_domain_2022_07_31_1711";
pub const SEED_TRANSFER: &str = "transfer_2022_07_31_1733";
pub const MAX_DOMAIN_ACCOUNT_LENGTH: usize = (4 + 32) + (4 + 30);
pub const MAX_TRANSFER_LENGTH: usize = 32;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Copy, PartialEq)]
pub enum Key {
    Uninitialized,
}

//====================Args====================

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct CreateDomainArgs {
    pub address: String,
    pub domain: String,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct TransferArgs {
    pub domain: String,
    pub amount: u64,
}

//====================Account====================

pub type DomainAccount = CreateDomainArgs;

impl DomainAccount {
    pub fn from_account_info(a: &AccountInfo) -> Result<DomainAccount, ProgramError> {
        if a.data_len() != MAX_DOMAIN_ACCOUNT_LENGTH {
            return Err(ProgramError::InvalidAccountData);
        }
        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}
