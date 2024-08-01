#![allow(clippy::arithmetic_side_effects)]
//! Program instruction processor

use std::cell::Ref;
use std::slice::Iter;
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, pubkey::Pubkey, system_instruction};
use solana_program::program_error::ProgramError;
use crate::assert_equal;
use crate::error::BonusPrizeError;
use crate::nll_state::DrawResult;
use crate::utils::constants::NO_LOSS_LOTTERY_ID;
use crate::utils::pdas::get_draw_result;

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if instruction_data.len() != 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let draw_number = u64::from_le_bytes(instruction_data[0..8].try_into().unwrap());
    let account_info_iter = &mut accounts.iter();
    let claimer = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;
    let vault_ata = next_account_info(account_info_iter)?;
    let claimer_ata = next_account_info(account_info_iter)?;
    let draw_result_account = next_account_info(account_info_iter)?;
    let lottery_account = next_account_info(account_info_iter)?;
    verify_draw_results_account(draw_number, *claimer.key, draw_result_account, lottery_account)?;
    Ok(())
}

fn verify_draw_results_account(draw_number: u64, claimer: Pubkey, draw_result_account: &AccountInfo, lottery_account: &AccountInfo) -> Result<(), ProgramError> {
    let data = draw_result_account.data.borrow();
    let draw_result_data: &DrawResult = bytemuck::from_bytes(&data);

    assert_equal!(draw_result_data.draw, draw_number, BonusPrizeError::DrawNumberMismatch);
    assert_equal!(draw_result_data.winner, claimer, BonusPrizeError::ClaimerNotWinner);

    let expected_draw_result_account = get_draw_result(draw_number, *lottery_account.key);
    assert_equal!(expected_draw_result_account, *draw_result_account.key, BonusPrizeError::DrawResultAccountDerivationError);

    assert_equal!(NO_LOSS_LOTTERY_ID, *draw_result_account.owner, BonusPrizeError::DrawResultAccountOwnerMismatch);
    Ok(())
}

