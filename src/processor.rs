#![allow(clippy::arithmetic_side_effects)]
//! Program instruction processor

use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, pubkey::Pubkey, system_instruction};
use solana_program::program::invoke_signed;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use crate::error::BonusPrizeError;
use crate::nll_state::DrawResult;

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
//    let draw_number = u64::from_le_bytes(instruction_data.into());
    let account_info_iter = &mut accounts.iter();
    let claimer = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;
    let vault_ata = next_account_info(account_info_iter)?;
    let claimer_ata = next_account_info(account_info_iter)?;
    let draw_result_account = next_account_info(account_info_iter)?;
    let data = draw_result_account.data.borrow();
    //let draw_result_data: &DrawResult = bytemuck::from_bytes(&data);
   // msg!("draw_result: {:?}", draw_result_data);
    msg!("Whatever");

    Ok(())
}

