use solana_program::decode_error::DecodeError;
use solana_program::msg;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

/// Errors that may be returned by the Bonus Prize program.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum BonusPrizeError {
    #[error("ClaimerNotWinner")]
    ClaimerNotWinner,
    #[error("DrawNumberMismatch")]
    DrawNumberMismatch,
    #[error("DrawResultAccountDerivationError")]
    DrawResultAccountDerivationError,
    #[error("DrawResultAccountOwnerMismatch")]
    DrawResultAccountOwnerMismatch,
    #[error("InvalidBonusPrizeSigner")]
    InvalidBonusPrizeSigner,
    #[error("DrawResultDisciminatorMismatch")]
    DrawResultDiscriminatorMismatch,
}
impl From<BonusPrizeError> for ProgramError {
    fn from(e: BonusPrizeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for BonusPrizeError {
    fn type_of() -> &'static str {
        "BonusPrizeError"
    }
}
impl PrintProgramError for BonusPrizeError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            BonusPrizeError::ClaimerNotWinner => msg!("Error: Claimer is not the winner"),
            BonusPrizeError::DrawNumberMismatch => msg!("Error: Draw number mismatch"),
            BonusPrizeError::DrawResultAccountDerivationError => {
                msg!("Error: Draw result account derivation error")
            }
            BonusPrizeError::DrawResultAccountOwnerMismatch => {
                msg!("Error: Draw result account owner mismatch")
            }
            BonusPrizeError::InvalidBonusPrizeSigner => msg!("Error: Invalid bonus prize signer"),
            BonusPrizeError::DrawResultDiscriminatorMismatch => {
                msg!("Error: Draw result discriminator mismatch")
            }
        }
    }
}
