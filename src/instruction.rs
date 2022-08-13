use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::{CreateDomainArgs, TransferArgs};

#[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum GameInstruction {
    CreateDomain(CreateDomainArgs),
    Transfer(TransferArgs),
    TransferToken(TransferArgs),
}
