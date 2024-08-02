use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use crate::ID;
use crate::utils::pdas::{get_bonus_prize_seed_signer, get_draw_result};

pub fn create_claim_instruction(
    claimer: Pubkey,
    mint: Pubkey,
    lottery: Pubkey,
    draw_number: u64,
) -> Instruction {

    let data = draw_number.to_le_bytes().to_vec();
    let draw_result_account = get_draw_result(draw_number, lottery);
    let bonus_prize_seed_signer = get_bonus_prize_seed_signer(draw_number, lottery);
    let vault_ata = get_associated_token_address(&bonus_prize_seed_signer, &mint);
    let claimer_ata = get_associated_token_address(&claimer, &mint);

    Instruction {
        program_id: ID,
        accounts: vec![
            AccountMeta::new(claimer, true),
            AccountMeta::new_readonly(bonus_prize_seed_signer, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(claimer_ata, false),
            AccountMeta::new(vault_ata, false),
            AccountMeta::new_readonly(draw_result_account, false),
            AccountMeta::new(lottery, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data,
    }
}
