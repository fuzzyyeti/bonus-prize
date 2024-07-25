use borsh::{BorshDeserialize, BorshSerialize, to_vec};
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use crate::state::PRIZE_MINTS_SEED;

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum Instructions {
    AddPrizes {
        number_of_prizes: u8,
    },
    SendPrizes {
        draw_number: u64,
    }
}

pub fn create_add_prizes_instruction(
    program_id: &Pubkey,
    draw_number: u64,
    draw_result_account: &Pubkey,
    prize_mints: Vec<Pubkey>,
) -> Instruction {
    let number_of_prizes = prize_mints.len() as u8;
    let data = to_vec(&Instructions::AddPrizes { number_of_prizes }).unwrap();
    let (prize_mints_account, _) = Pubkey::find_program_address(
        &[PRIZE_MINTS_SEED, &draw_result_account.to_bytes(), &draw_number.to_le_bytes()],
        program_id);

    let mut accounts = vec![
        AccountMeta::new(prize_mints_account, false),
        AccountMeta::new(*draw_result_account, false)
        ];
    accounts.extend(prize_mints.iter().map(|mint| AccountMeta::new_readonly(*mint, false)));

    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}


