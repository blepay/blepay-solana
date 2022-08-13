use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, pubkey::Pubkey};
use solana_program::program::invoke;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::instruction::transfer;

use crate::state::TransferArgs;

pub fn process_transfer_token(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: TransferArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let signer_ata_info = next_account_info(account_info_iter)?;
    let target_info = next_account_info(account_info_iter)?;
    let target_ata_info = next_account_info(account_info_iter)?;
    let token_mint_info = next_account_info(account_info_iter)?;
    let ass_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    if target_info.lamports() <= 0 {
        msg!("Create Associated Token Account");
        invoke(
            &create_associated_token_account(
                signer_info.key,
                target_info.key,
                token_mint_info.key,
            ),
            &[
                signer_info.clone(),
                target_info.clone(),
                target_ata_info.clone(),
                token_mint_info.clone(),
                token_program_info.clone(),
                ass_program_info.clone(),
                system_info.clone(),
            ],
        )?;
    }

    msg!("Transfer Token");
    invoke(
        &transfer(
            token_program_info.key,
            signer_ata_info.key,
            target_ata_info.key,
            signer_info.key,
            &[signer_info.key],
            args.amount,
        )?,
        &[
            signer_ata_info.clone(),
            target_ata_info.clone(),
            signer_info.clone(),
            token_program_info.clone()
        ],
    )?;

    Ok(())
}
