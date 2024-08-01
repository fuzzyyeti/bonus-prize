//! A program demonstrating the transfer of lamports
#![forbid(unsafe_code)]

use solana_program::pubkey;
use solana_program::pubkey::Pubkey;

pub const ID: Pubkey = pubkey!("54oykPNNXxpXihbuU5H6j3MZmqCxaAdHALDvVYfzwnW4");

mod entrypoint;
pub mod processor;
mod error;
pub mod utils;
mod nll_state;
pub mod instruction;

