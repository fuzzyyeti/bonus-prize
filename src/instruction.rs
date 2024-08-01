use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use crate::ID;
use crate::utils::pdas::get_draw_result;

pub fn create_claim_instruction(
    claimer: Pubkey,
    mint: Pubkey,
    lottery: Pubkey,
    draw_number: u64,
) -> Instruction {

    let draw_result_account = get_draw_result(draw_number, lottery);
    let claimer = Pubkey::default();
    let vault_ata = Pubkey::default();
    let claimer_ata = Pubkey::default();
    let mint = Pubkey::default();

    Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new(claimer, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(vault_ata, false),
            AccountMeta::new(claimer_ata, false),
            AccountMeta::new(draw_result_account, false),
        ],
        data: vec![0],
    }
}