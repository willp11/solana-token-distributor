use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum SolBetError {
        // Invalid instruction
        #[error("Invalid Instruction")]
        InvalidInstruction
}

impl From<SolBetError> for ProgramError {
    fn from(e: SolBetError) -> Self {
        ProgramError::Custom(e as u32)
    }
}