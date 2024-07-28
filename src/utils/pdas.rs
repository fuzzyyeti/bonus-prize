use solana_program::pubkey;
use solana_program::pubkey::Pubkey;

pub const NO_LOSS_LOTTERY_ID: Pubkey = pubkey!("57JfdST1qV2upu9fU3E2K2GdQpzJhU36C8n61qnZrGea");
pub const DRAW_RESULT: &[u8] = b"draw_result";
pub const LOTTERY_ACCOUNT: Pubkey = pubkey!("2pqqzZMoFpXem9tVMoWCz1xfQiZp9863479ta739Dybm");

pub fn get_draw_results_pda(draw_number: u64, lottery_account: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[DRAW_RESULT, &draw_number.to_le_bytes(), &lottery_account.to_bytes()], &NO_LOSS_LOTTERY_ID).0
}