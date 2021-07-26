use solana_program::program_error::ProgramError;
use crate::error::TokenDistributorError::InvalidInstruction;
use std::convert::TryInto;

pub enum TokenDistributorInstruction {

    // Accounts expected:
    // 0. [signer] initializer
    // 1. [writable] lockup schedule state (empty)
    // 2. [] token mint
    // 3. [] clock sysvar
    // 4. [] rent sysvar
    CreateLockupSchedule {
        start_timestamp: u64,
        total_unlock_periods: u64,
        period_duration: u64, // in seconds
        total_lockup_quantity: u64,
    },

    // Accounts expected:
    // 0. [signer] initializer
    // 1. [writable] lockup schedule state
    // 2. [writable] lockup state account (empty)
    // 3. [] token receiver main Solana account
    // 4. [writable] temporary lockup token account
    // 5. [] token program (transfer ownership of temp token account to PDA)
    // 6. [] clock sysvar
    // 7. [] rent sysvar
    LockTokens {
        token_quantity: u64,
    },

    // Accounts expected:
    // 0. [signer] token receiver's main Solana account
    // 1. [] lockup schedule state
    // 2. [writable] lockup state
    // 3. [writable] lockup token account
    // 4. [writable] receiving token account
    // 5. [signer] program-derived-address (owns lockup token account)
    // 6. [] token program
    // 7. [] clock sysvar
    RedeemTokens {

    }
}

impl TokenDistributorInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => Self::CreateLockupSchedule {
                start_timestamp: Self::unpack_start_timestamp(rest)?,
                total_unlock_periods: Self::unpack_total_unlock_periods(rest)?,
                period_duration: Self::unpack_period_duration(rest)?,
                total_lockup_quantity: Self::unpack_total_lockup_quantity(rest)?
            }, 
            1 => Self::LockTokens {
                token_quantity: Self::unpack_token_quantity(rest)?
            },
            2 => Self::RedeemTokens {},
            _ => return Err(InvalidInstruction.into())
        })
    }

    // unpack CreateLockupSchedule data
    fn unpack_start_timestamp(input: &[u8]) -> Result<u64, ProgramError> {
        let start_timestamp = input.get(..8).and_then(|slice| slice.try_into().ok()).map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
        Ok(start_timestamp)
    }
    fn unpack_total_unlock_periods(input: &[u8]) -> Result<u64, ProgramError> {
        let total_unlock_periods = input.get(8..16).and_then(|slice| slice.try_into().ok()).map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
        Ok(total_unlock_periods)
    }
    fn unpack_period_duration(input: &[u8]) -> Result<u64, ProgramError> {
        let period_duration = input.get(16..24).and_then(|slice| slice.try_into().ok()).map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
        Ok(period_duration)
    }
    fn unpack_total_lockup_quantity(input: &[u8]) -> Result<u64, ProgramError> {
        let total_lockup_quantity = input.get(24..32).and_then(|slice| slice.try_into().ok()).map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
        Ok(total_lockup_quantity)
    }

    // unpack LockTokens data
    fn unpack_token_quantity(input: &[u8]) -> Result<u64, ProgramError> {
        let token_quantity = input.get(..8).and_then(|slice| slice.try_into().ok()).map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
        Ok(token_quantity)
    }
}