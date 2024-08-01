use solana_program::pubkey;
use solana_program::pubkey::Pubkey;
use crate::ID;

pub const NO_LOSS_LOTTERY_ID: Pubkey = pubkey!("57JfdST1qV2upu9fU3E2K2GdQpzJhU36C8n61qnZrGea");
pub const BONUS_PRIZE: &[u8] = b"bonus_prize";

pub const LOTTERY_ACCOUNT: Pubkey = pubkey!("2pqqzZMoFpXem9tVMoWCz1xfQiZp9863479ta739Dybm");

pub const DRAW_RESULT: &[u8] = b"draw_result";

pub fn get_prize_mints(draw_number: u64, lottery_account: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[BONUS_PRIZE, &lottery_account.to_bytes(), &draw_number.to_le_bytes()], &ID).0
}

pub fn get_draw_result(draw_number: u64, lottery_account: Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[DRAW_RESULT, &draw_number.to_le_bytes(), &lottery_account.to_bytes()], &NO_LOSS_LOTTERY_ID).0
}
