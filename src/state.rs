use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

use crate::error::TokenDistributorError;

// LOCKUP SCHEDULE STATE
pub struct LockupSchedule {
    is_initialized: bool,
    token_mint: Pubkey,
    start_timestamp: u64,
    number_periods: u32,
    period_duration: u64,
    total_token_quantity: u64,
    token_quantity_locked: u64
}

impl Sealed for LockupSchedule {}

impl IsInitialized for LockupSchedule {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for LockupSchedule {
    // is_intialized=1, mint=4, start_timestamp=8, number_periods=4, duration=8, total_quantity=8, quantity_locked=8
    const LEN: usize = 41;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, LockupSchedule::LEN];
        let (
            is_initialized,
            token_mint,
            start_timestamp,
            number_periods,
            period_duration,
            total_token_quantity,
            token_quantity_locked
        ) = array_refs![src, 1, 4, 8, 4, 8, 8, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(TokenDistributorError::InvalidLockupScheduleData.into());
        }

        Ok(LockupSchedule {
            is_initialized,
            token_mint: Pubkey::new_from_array(*token_mint),
            start_timestamp: u64::from_le_bytes(*start_timestamp),
            number_periods: u32::from_le_bytes(*number_periods),
            period_duration: u64::from_le_bytes(*period_duration),
            total_token_quantity: u64::from_le_bytes(*total_token_quantity),
            token_quantity_locked: u64::from_le_bytes(*token_quantity_locked)
        })
    }

    fn pack_into_slice(&self, dstL &mut [u8]) {
        let dst = array_mut_ref![dst, 0, LockupSchedule::LEN];
        let (
            is_initialized_dst,
            token_mint_dst,
            start_timestamp_dst,
            number_periods_dst,
            period_duration_dst,
            total_token_quantity_dst,
            token_quantity_locked_dst
        ) = mut_array_refs![dst, 1, 4, 8, 4, 8, 8, 8];

        let LockupSchedule {
            is_initialized,
            token_mint,
            start_timestamp,
            number_periods,
            period_duration,
            total_token_quantity,
            token_quantity_locked
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        token_mint_dst.copy_from_slice(token_mint.as_ref());
        *start_timestamp_dst = start_timestamp.to_le_bytes();
        *number_periods_dst = number_periods.to_le_bytes();
        *period_duration_dst = period_duration.to_le_bytes();
        *total_token_quantity_dst = total_token_quantity.to_le_bytes();
        *token_quantity_locked_dst = token_quantity_locked.to_le_bytes();
    }

}