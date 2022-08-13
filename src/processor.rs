use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub use create_domain::*;
pub use transfer::*;
pub use transfer_token::*;

use crate::instruction::*;

pub mod create_domain;
pub mod transfer;
pub mod transfer_token;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = GameInstruction::try_from_slice(input)?;
    match instruction {
        GameInstruction::CreateDomain(args) => {
            process_create_domain(program_id, accounts, args)
        }
        GameInstruction::Transfer(args) => {
            process_transfer(program_id, accounts, args)
        }
        GameInstruction::TransferToken(args) => {
            process_transfer_token(program_id, accounts, args)
        }
    }
}
