use crate::utils::constants::{BONUS_PRIZE, DRAW_RESULT, NO_LOSS_LOTTERY_ID};
use crate::ID;
use solana_program::pubkey;
use solana_program::pubkey::Pubkey;

pub fn get_bonus_prize_seed_signer(draw_number: u64, lottery_account: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            BONUS_PRIZE,
            &lottery_account.to_bytes(),
            &draw_number.to_le_bytes(),
        ],
        &ID,
    )
    .0
}

pub fn get_draw_result(draw_number: u64, lottery_account: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            DRAW_RESULT,
            &draw_number.to_le_bytes(),
            &lottery_account.to_bytes(),
        ],
        &NO_LOSS_LOTTERY_ID,
    )
    .0
}
