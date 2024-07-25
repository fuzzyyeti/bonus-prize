use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub const PRIZE_MINTS_SEED : &[u8] = b"prize_mints";

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct PrizeMints {
    pub mints: Vec<Pubkey>
}
