use bytemuck::{Pod, Zeroable};
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct DrawResult {
    pub discriminator: [u8; 8],
    pub winner: Pubkey,
    pub draw: u64,
    pub timestamp: i64,
    pub prize: u64,
    /// Draw result version. always 1
    pub version: u8,
    pub _reserved0: [u8; 7],
    pub lottery_num_tickets: u64,
    pub winning_ticket: u64,
    pub winner_num_tickets: u64,
}
