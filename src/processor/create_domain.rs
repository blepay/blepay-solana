use borsh::BorshSerialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use crate::state::{CreateDomainArgs, DomainAccount, MAX_DOMAIN_ACCOUNT_LENGTH, SEED_BLEPAY, SEED_CREATE_DOMAIN};
use crate::utils::{assert_derivation, create_or_allocate_account_raw};

pub fn process_create_domain(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: CreateDomainArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let domain_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    let bump_seed = assert_derivation(
        program_id,
        domain_info,
        &[
            SEED_BLEPAY.as_bytes(),
            program_id.as_ref(),
            SEED_CREATE_DOMAIN.as_bytes(),
            args.domain.as_bytes(),
        ],
    )?;
    let seeds = &[
        SEED_BLEPAY.as_bytes(),
        program_id.as_ref(),
        SEED_CREATE_DOMAIN.as_bytes(),
        args.domain.as_bytes(),
        &[bump_seed],
    ];
    create_or_allocate_account_raw(
        *program_id,
        domain_info,
        rent_info,
        system_info,
        signer_info,
        MAX_DOMAIN_ACCOUNT_LENGTH,
        seeds,
    )?;

    let mut domain_account = DomainAccount::from_account_info(domain_info)?;
    domain_account.address = args.address;
    domain_account.domain = args.domain;
    domain_account.serialize(&mut *domain_info.try_borrow_mut_data()?)?;

    Ok(())
}
