#![allow(clippy::arithmetic_side_effects)]
//! Program instruction processor

use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, pubkey::Pubkey, system_instruction};
use solana_program::program::invoke_signed;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use crate::error::BonusPrizeError;
use crate::instructions::Instructions;
use crate::instructions::Instructions::{AddPrizes, SendPrizes};
use borsh::{BorshDeserialize, to_vec};
use crate::state::PrizeMints;

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    // As part of the program specification the first account is the source
    // account and the second is the destination account

    let discriminator = Instructions::try_from_slice(instruction_data);
    return match discriminator {
        Ok(AddPrizes{  number_of_prizes }) => {
            process_add_prize(number_of_prizes, accounts, program_id)
        }
        Ok(SendPrizes { draw_number}) => {
            process_send_prize(draw_number, accounts)
        }
        _ => {
            Err(BonusPrizeError::InvalidInstruction.into())
        }
    }
}

fn process_send_prize(_draw_number: u64, _accounts: &[AccountInfo]) -> ProgramResult {
    todo!()
}

/// Process `AddPrizes` instruction
pub fn process_add_prize(number_of_prizes: u8, accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    // Create an iterator to safely reference accounts in the slice
    let account_info_iter = &mut accounts.iter();
    let mut prizes: Vec<Pubkey> = Vec::new();

    let prize_mints_account = next_account_info(account_info_iter)?;
    let draw_result_account = next_account_info(account_info_iter)?;

    for _ in 0..number_of_prizes {
        let next_account_info = next_account_info(account_info_iter)?;
        prizes.push(*next_account_info.key);
    }
    let prize_mints = PrizeMints { mints: prizes };
    let serialized_prize_mints_data = to_vec(&prize_mints)?;
    let space = serialized_prize_mints_data.len();
    let rent_exempt_reserve = Rent::get()?.minimum_balance(space);
    // Create prize_mint account
    invoke_signed(
        &system_instruction::create_account(
            &prize_mints_account.key,
            program_id,
            rent_exempt_reserve,
            space as u64,
            program_id,
        ),
        &[prize_mints_account.clone()],
        &[&[&b"prize_mint"[..], &draw_result_account.key.to_bytes(), &[0u8]]],
    )?;

    prize_mints_account.data.borrow_mut()[..space].copy_from_slice(&serialized_prize_mints_data);
    Ok(())
}
