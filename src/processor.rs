#![allow(clippy::arithmetic_side_effects)]
//! Program instruction processor

use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, pubkey::Pubkey, system_instruction};
use solana_program::program::invoke_signed;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use crate::error::BonusPrizeError;
use crate::instructions::Instructions;
use crate::instructions::Instructions::{AddPrizes, SendPrizes};
use borsh::{BorshDeserialize, to_vec};
use crate::state::{PRIZE_MINTS_SEED, PrizeMints};

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
        Ok(AddPrizes{  number_of_prizes, draw_number }) => {
            process_add_prize(number_of_prizes, draw_number, accounts, program_id)
        }
        Ok(SendPrizes { draw_number}) => {
            process_send_prize(draw_number, accounts)
        }
        _ => {
            Err(BonusPrizeError::InvalidInstruction.into())
        }
    }
}

fn process_send_prize(draw_number: u64, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let payer = next_account_info(account_info_iter)?;
    let prize_mints_account = next_account_info(account_info_iter)?;
    let draw_result_account = next_account_info(account_info_iter)?;
    let data = &draw_result_account.data.borrow()[8..];
    // Extract the winner Pubkey (32 bytes)
    let winner_bytes: [u8; 32] = data[0..32].try_into().unwrap();
    let winner = Pubkey::new_from_array(winner_bytes);
    // Extract the draw (u64, 8 bytes)
    let draw_bytes: [u8; 8] = data[32..40].try_into().unwrap();
    let draw = u64::from_le_bytes(draw_bytes);
    msg!("winner: {:?} draw: {:?}", winner, draw);
    Ok(())

}

/// Process `AddPrizes` instruction
pub fn process_add_prize(number_of_prizes: u8, draw_number: u64, accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    // Create an iterator to safely reference accounts in the slice
    let account_info_iter = &mut accounts.iter();
    let mut prizes: Vec<Pubkey> = Vec::new();

    let payer = next_account_info(account_info_iter)?;
    let prize_mints_account = next_account_info(account_info_iter)?;
    let lottery_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let (_prize_mint_address, bump) = Pubkey::find_program_address(
        &[PRIZE_MINTS_SEED, &lottery_account.key.to_bytes(), &draw_number.to_le_bytes()],
        program_id);

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
            &payer.key,
            &prize_mints_account.key,
            rent_exempt_reserve,
            space as u64,
            program_id,
        ),
        &[payer.clone(), prize_mints_account.clone(), system_program.clone()],
        &[&[&PRIZE_MINTS_SEED, &lottery_account.key.to_bytes(), &draw_number.to_le_bytes(), &[bump]]],
    )?;

    prize_mints_account.data.borrow_mut()[..space].copy_from_slice(&serialized_prize_mints_data);
    Ok(())
}
