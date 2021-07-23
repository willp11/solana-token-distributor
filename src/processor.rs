use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack, IsInitialized},
    sysvar::{rent::Rent, Sysvar},
    program::{invoke, invoke_signed},
    clock::{Clock}
};

use crate::{instruction::TokenDistributorInstruction, state::LockupSchedule, state::Lockup}

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        
        let instruction = TokenDistributorInstruction::unpack(instruction_data)?;
        match instruction {
            TokenDistributorInstruction::CreateLockupSchedule {start_timestamp, total_unlock_periods, period_duration, total_lockup_quantity} => {
                msg!("Instruction: CreateLockupSchedule");
                Self::process_create_lockup_schedule(accounts, program_id, start_timestamp, total_unlock_periods, period_duration, total_lockup_quantity)
            },
            TokenDistributorInstruction::LockTokens {token_quantity} => {
                msg!("Instruction: LockTokens");
                Self::process_lock_tokens(accounts, token_quantity, program_id)
            },
            TokenDistributorInstruction::RedeemTokens {} => {
                msg!("Instruction: RedeemTokens");
                Self::process_redeem_tokens(accounts, program_id)
            },
            _ => return Err(ProgramError::InvalidInstructionData);
        }
    }

    // CREATE LOCKUP SCHEDULE
    fn process_create_lockup_schedule(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        start_timestamp: u64,
        total_unlock_periods: u32,
        period_duration: u64,
        total_lockup_quantity: u64
    ) -> ProgramResult {

        // get accounts
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;
        let lockup_schedule_state_account = next_account_info(account_info_iter)?;
        let token_mint = next_account_info(account_info_iter)?;
        let clock_sysvar = next_account_info(account_info_iter)?;
        let rent_sysvar = next_account_info(account_info_iter)?;

        // check the start timestamp is after current timestamp
        let clock = &Clock::from_account_info(clock_sysvar)?;
        let current_timestamp = clock.unix_timestamp as u64;
        if current_timestamp > start_timestamp {
            return Err(TokenDistributorError::InvalidStartTimestamp.into());
        }
        
        // check empty state account has enough lamports
        if !rent.is_exempt(lockup_schedule_state_account.lamports(), lockup_schedule_state_account.data_len()) {
            return Err(TokenDistributorError::NotRentExempt.into());
        }

        // write lockup information to state account
        let mut lockup_schedule_state = LockupSchedule::unpack_unchecked(&lockup_schedule_state_account.data.borrow())?;
        lockup_schedule_state.is_initialized = true;
        lockup_schedule_state.token_mint = *token_mint.key;
        lockup_schedule_state.start_timestamp = start_timestamp;
        lockup_schedule_state.number_periods = total_unlock_periods;
        lockup_schedule_state.period_duration = period_duration;
        lockup_schedule_state.total_token_quantity = total_lockup_quantity;
        lockup_schedule_state.token_quantity_locked = 0;
        LockupSchedule::pack(lockup_schedule_state, &mut lockup_schedule_state_account.data.borrow_mut())?;

        Ok(())
    }

    // LOCK TOKENS
    fn process_lock_tokens(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        token_quantity: u64
    ) -> ProgramResult {

        Ok(())
    }

    // REDEEM TOKENS 
    fn process_redeem_tokens(
        accounts: &[AccountInfo],
        program_id: &Pubkey
    ) -> ProgramResult {

        Ok(())
    }
}