use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use crate::state::{PRIZE_MINTS_SEED, PrizeMints};
use crate::utils::pdas::get_prize_mints;

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum Instructions {
    AddPrizes {
        number_of_prizes: u8,
        draw_number: u64,
    },
    SendPrizes {
        draw_number: u64,
    }
}


pub fn send_prizes(
    program_id: &Pubkey,
    draw_number: u64,
    lottery_account: Pubkey,
    payer: Pubkey,
) -> Instruction {
todo!();
}

pub fn create_add_prizes_instruction(
    program_id: &Pubkey,
    draw_number: u64,
    lottery_account: Pubkey,
    prize_mints: Vec<Pubkey>,
    payer: Pubkey,
) -> Instruction {
    let number_of_prizes = prize_mints.len() as u8;
    let data = to_vec(&Instructions::AddPrizes { number_of_prizes, draw_number }).unwrap();
    let prize_mints_account = get_prize_mints(draw_number, lottery_account);

    let mut accounts = vec![
        AccountMeta::new(payer, true),
        AccountMeta::new(prize_mints_account, false),
        AccountMeta::new(lottery_account, false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ];
    accounts.extend(prize_mints.iter().map(|mint| AccountMeta::new_readonly(*mint, false)));

    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}


