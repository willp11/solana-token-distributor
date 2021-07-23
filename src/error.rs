use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum TokenDistributorError {
        // Invalid instruction
        #[error("Invalid Instruction")]
        InvalidInstruction,
        // Invalid lockup schedule data
        #[error("Invalid lockup schedule data")]
        InvalidLockupScheduleData
}

impl From<TokenDistributorError> for ProgramError {
    fn from(e: TokenDistributorError) -> Self {
        ProgramError::Custom(e as u32)
    }
}