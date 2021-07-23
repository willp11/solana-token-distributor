use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum TokenDistributorError {
        // Invalid instruction
        #[error("Invalid Instruction")]
        InvalidInstruction,
        // Invalid lockup schedule data
        #[error("Invalid lockup schedule data")]
        InvalidLockupScheduleData,
        // Not rent exempt
        #[error("State account not rent exempt")]
        NotRentExempt,
        // Invalid start timestamp
        #[error("Invalid start timestamp")]
        InvalidStartTimestamp,
        // Invalid mint
        #[error("Invalid mint")]
        InvalidMint,
        // Expected amount mismatch - wrong number of tokens in temporary token account
        #[error("Expected amount mismatch")]
        ExpectedAmountMismatch
}

impl From<TokenDistributorError> for ProgramError {
    fn from(e: TokenDistributorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}