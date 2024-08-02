#![allow(clippy::arithmetic_side_effects)]
//! Program instruction processor

use std::cell::Ref;
use std::slice::Iter;
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, pubkey::Pubkey, system_instruction};
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::program_pack::Pack;
use spl_token::instruction::{transfer, transfer_checked};
use spl_token::state::Account;
use crate::assert_equal;
use crate::error::BonusPrizeError;
use crate::nll_state::DrawResult;
use crate::utils::constants::{BONUS_PRIZE, NO_LOSS_LOTTERY_ID};
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
    let bonus_prize_seed_signer = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;
    let claimer_ata = next_account_info(account_info_iter)?;
    let vault_ata = next_account_info(account_info_iter)?;
    let draw_result_account = next_account_info(account_info_iter)?;
    let lottery_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;


    // Verify accounts
    verify_draw_results_account(draw_number, *claimer.key, draw_result_account, *lottery_account.key)?;


    let (expected_bonus_prize, bump) = Pubkey::find_program_address(
        &[BONUS_PRIZE, &lottery_account.key.to_bytes(), &draw_number.to_le_bytes()],
        program_id,
    );

    if expected_bonus_prize != *bonus_prize_seed_signer.key {
        return Err(BonusPrizeError::InvalidBonusPrizeSigner.into());
    }

    // Transfer the prize from the vault to the winner. All tokens in the ATA

    let vault_account_data: Account;
    {
        let data = vault_ata.try_borrow_data()?;
        vault_account_data = Account::unpack(&data)?;
    }

    let transfer_ix = transfer(
        token_program.key,
        &vault_ata.key,
        &claimer_ata.key,
        &bonus_prize_seed_signer.key,
        &[],
        vault_account_data.amount)?;

    invoke_signed(
        &transfer_ix,
        &[
            vault_ata.clone(),
            claimer_ata.clone(),
            bonus_prize_seed_signer.clone(),
        ],
        &[&[&BONUS_PRIZE, &lottery_account.key.to_bytes(), &draw_number.to_le_bytes(), &[bump]],],
    )?;
    Ok(())
}

fn verify_draw_results_account(draw_number: u64, claimer: Pubkey, draw_result_account: &AccountInfo, lottery_account_key: Pubkey) -> Result<(), ProgramError> {
    let data = draw_result_account.data.borrow();
    let draw_result_data: &DrawResult = bytemuck::from_bytes(&data);

    assert_equal!(draw_result_data.draw, draw_number, BonusPrizeError::DrawNumberMismatch);
    assert_equal!(draw_result_data.winner, claimer, BonusPrizeError::ClaimerNotWinner);

    let expected_draw_result_account = get_draw_result(draw_number, lottery_account_key);
    assert_equal!(expected_draw_result_account, *draw_result_account.key, BonusPrizeError::DrawResultAccountDerivationError);

    assert_equal!(NO_LOSS_LOTTERY_ID, *draw_result_account.owner, BonusPrizeError::DrawResultAccountOwnerMismatch);
    Ok(())
}

