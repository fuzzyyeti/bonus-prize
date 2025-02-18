use solana_program::pubkey;
use solana_program::pubkey::Pubkey;

pub const NO_LOSS_LOTTERY_ID: Pubkey = pubkey!("57JfdST1qV2upu9fU3E2K2GdQpzJhU36C8n61qnZrGea");
pub const BONUS_PRIZE: &[u8] = b"bonus_prize";

pub const LOTTERY_ACCOUNT: Pubkey = pubkey!("2pqqzZMoFpXem9tVMoWCz1xfQiZp9863479ta739Dybm");

pub const DRAW_RESULT: &[u8] = b"draw_result";

pub const DRAW_RESULT_DISCRIMINATOR: [u8; 8] = [0x39,0xb6,0xbd,0x10,0x63,0xe8,0x3b,0xf3];
