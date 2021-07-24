use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

use crate::error::TokenDistributorError;

// LOCKUP SCHEDULE STATE
pub struct LockupSchedule {
    pub is_initialized: bool,
    pub token_mint: Pubkey,
    pub start_timestamp: u64,
    pub number_periods: u32,
    pub period_duration: u64,
    pub total_token_quantity: u64,
    pub token_quantity_locked: u64
}

impl Sealed for LockupSchedule {}

impl IsInitialized for LockupSchedule {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for LockupSchedule {
    // is_intialized=1, mint=32, start_timestamp=8, number_periods=4, duration=8, total_quantity=8, quantity_locked=8
    const LEN: usize = 69;

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
        ) = array_refs![src, 1, 32, 8, 4, 8, 8, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(TokenDistributorError::InvalidLockupScheduleData.into())
        };

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

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, LockupSchedule::LEN];
        let (
            is_initialized_dst,
            token_mint_dst,
            start_timestamp_dst,
            number_periods_dst,
            period_duration_dst,
            total_token_quantity_dst,
            token_quantity_locked_dst
        ) = mut_array_refs![dst, 1, 32, 8, 4, 8, 8, 8];

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

// LOCKUP STATE
pub struct Lockup {
    pub is_initialized: bool,
    pub lockup_schedule_state: Pubkey,
    pub receiving_account: Pubkey,
    pub lockup_token_account: Pubkey,
    pub token_quantity: u64,
    pub periods_redeemed: u32
}

impl Sealed for Lockup {}

impl IsInitialized for Lockup {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Lockup {
    // is_intialized=1, lockup_schedule_state=32, receiving_account=32, lockup_token_account=32, token_quantity=8, periods_redeemed=4
    const LEN: usize = 109;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Lockup::LEN];
        let (
            is_initialized,
            lockup_schedule_state,
            receiving_account,
            lockup_token_account,
            token_quantity,
            periods_redeemed
        ) = array_refs![src, 1, 32, 32, 32, 8, 4];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(TokenDistributorError::InvalidLockupScheduleData.into())
        };

        Ok(Lockup {
            is_initialized,
            lockup_schedule_state: Pubkey::new_from_array(*lockup_schedule_state),
            receiving_account: Pubkey::new_from_array(*receiving_account),
            lockup_token_account: Pubkey::new_from_array(*lockup_token_account),
            token_quantity: u64::from_le_bytes(*token_quantity),
            periods_redeemed: u32::from_le_bytes(*periods_redeemed)
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Lockup::LEN];
        let (
            is_initialized_dst,
            lockup_schedule_state_dst,
            receiving_account_dst,
            lockup_token_account_dst,
            token_quantity_dst,
            periods_redeemed_dst
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8, 4];

        let Lockup {
            is_initialized,
            lockup_schedule_state,
            receiving_account,
            lockup_token_account,
            token_quantity,
            periods_redeemed
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        lockup_schedule_state_dst.copy_from_slice(lockup_schedule_state.as_ref());
        receiving_account_dst.copy_from_slice(receiving_account.as_ref());
        lockup_token_account_dst.copy_from_slice(lockup_token_account.as_ref());
        *token_quantity_dst = token_quantity.to_le_bytes();
        *periods_redeemed_dst = periods_redeemed.to_le_bytes();
    }
}