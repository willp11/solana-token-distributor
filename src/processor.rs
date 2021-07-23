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

use spl_token::state::Account as TokenAccount;

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

        // get accounts
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;
        let lockup_schedule_state_account = next_account_info(account_info_iter)?;
        let empty_state_account = next_account_info(account_info_iter)?;
        let receiver_account = next_account_info(account_info_iter)?;
        let temp_token_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        let clock_sysvar = next_account_info(account_info_iter)?;
        let rent_sysvar = next_account_info(account_info_iter)?;

        // check the initializer signed the tx
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // check empty state account has enough lamports to ensure it doesn't get closed by the runtime
        if !rent.is_exempt(empty_state_account.lamports(), empty_state_account.data_len()) {
            return Err(TokenDistributorError::NotRentExempt.into());
        }

        // unpack the lockup schedule state
        let mut lockup_schedule_state = LockupSchedule::unpack_unchecked(&lockup_schedule_state_account.data.borrow())?;
        
        // check current time is before lockup start time
        let clock = &Clock::from_account_info(clock_sysvar)?;
        let current_timestamp = clock.unix_timestamp as u64;
        if current_timestamp > lockup_schedule_state.start_timestamp {
            return Err(TokenDistributorError::InvalidStartTimestamp.into());
        }

        // unpack temp token account data
        let temp_token_account_info = TokenAccount::unpack(&temp_token_account.data.borrow())?;

        // check temp token account has same mint as written in lockup schedule state
        if temp_token_account_info.mint != lockup_schedule_state.token_mint {
            return Err(TokenDistributorError::InvalidMint.into());
        }

        // check number of tokens in temp account is the same as token_quantity
        if temp_token_account_info.amount != token_quantity {
            return Err(TokenDistributorError::ExpectedAmountMismatch.into());
        }
        
        // transfer ownership of temp token account to program-derived address
        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"tokenDistributor"], program_id);
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            initializer.key,
            &[&initializer.key],
        )?;
        msg!("Calling the token program to transfer token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ],
        )?;

        // unpack empty state account
        let mut lockup_state = Lockup::unpack_unchecked(&lockup_state_account.data.borrow())?;

        // write lockup information to the empty state account
        lockup_state.is_initialized = true;
        lockup_state.lockup_schedule_state = *lockup_schedule_state_account.key;
        lockup_state.receiving_account = *receiver_account.key;
        lockup_state.lockup_token_account = *temp_token_account.key;
        lockup_state.token_quantity = token_quantity;
        lockup_state.periods_redeemed = 0;

        // update the token_quantity_locked variable in lockup schedule state
        lockup_schedule_state.token_quantity_locked += token_quantity;

        // pack the state accounts
        Lockup::pack(lockup_state, &mut lockup_state_account.data.borrow_mut())?;
        LockupSchedule::pack(lockup_schedule_state, &mut lockup_schedule_state_account.data.borrow_mut())?;

        Ok(())
    }

    // REDEEM TOKENS 
    fn process_redeem_tokens(
        accounts: &[AccountInfo],
        program_id: &Pubkey
    ) -> ProgramResult {

        // // get accounts
        // let account_info_iter = &mut accounts.iter();
        // let receiving_account = next_account_info(account_info_iter)?;

        // // check the initializer signed the tx
        // if !receiving_account.is_signer {
        //     return Err(ProgramError::MissingRequiredSignature);
        // }

        Ok(())
    }
}