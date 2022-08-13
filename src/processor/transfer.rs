use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, pubkey::Pubkey, system_instruction};
use solana_program::program::invoke;

use crate::state::TransferArgs;

pub fn process_transfer(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: TransferArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let target_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    invoke(
        &system_instruction::transfer(
            &signer_info.key,
            &target_info.key,
            args.amount),
        &[
            signer_info.clone(),
            target_info.clone(),
            system_info.clone(),
        ],
    )?;

    Ok(())
}
