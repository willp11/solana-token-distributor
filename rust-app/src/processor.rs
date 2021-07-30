use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
    program_pack::{Pack},
    sysvar::{rent::Rent, Sysvar},
    program::{invoke, invoke_signed},
    clock::{Clock}
};

use std::cmp;

use spl_token::state::Account as TokenAccount;

use crate::{instruction::TokenDistributorInstruction, state::LockupSchedule, state::Lockup, error::TokenDistributorError};

pub struct Processor;
impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        
        let instruction = TokenDistributorInstruction::unpack(instruction_data)?;
        match instruction {
            TokenDistributorInstruction::CreateLockupSchedule {start_timestamp, total_unlock_periods, period_duration, total_lockup_quantity } => {
                msg!("Instruction: CreateLockupSchedule");
                Self::process_create_lockup_schedule(accounts, start_timestamp, total_unlock_periods, period_duration, total_lockup_quantity, program_id)
            },
            TokenDistributorInstruction::LockTokens {token_quantity} => {
                msg!("Instruction: LockTokens");
                Self::process_lock_tokens(accounts, token_quantity, program_id)
            },
            TokenDistributorInstruction::RedeemTokens {} => {
                msg!("Instruction: RedeemTokens");
                Self::process_redeem_tokens(accounts, program_id)
            }
        }
    }

    fn process_create_lockup_schedule(
        accounts: &[AccountInfo],
        start_timestamp: u64,
        total_unlock_periods: u64,
        period_duration: u64,
        total_lockup_quantity: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {

        // get accounts
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;
        let lockup_schedule_state_account = next_account_info(account_info_iter)?;
        let token_mint = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        msg!("checkpoint - 1");
        // check the initializer signed the tx
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // check the start timestamp is after current timestamp
        let current_timestamp = clock.unix_timestamp as u64;
        if current_timestamp > start_timestamp {
            return Err(TokenDistributorError::InvalidStartTimestamp.into());
        }
        msg!("checkpoint - 2");
        msg!("lockup_schedule_state_account.owner: {:?}", lockup_schedule_state_account.owner);
        msg!("program_id: {:?}", program_id);
        // check program is owner of state account
        if lockup_schedule_state_account.owner != program_id {
            return Err(TokenDistributorError::IncorrectOwner.into());
        }
        
        // check empty state account has enough lamports
        if !rent.is_exempt(lockup_schedule_state_account.lamports(), lockup_schedule_state_account.data_len()) {
            return Err(TokenDistributorError::NotRentExempt.into());
        }
        msg!("checkpoint - 3");
        // write lockup information to state account
        let mut lockup_schedule_state = LockupSchedule::unpack_unchecked(&lockup_schedule_state_account.data.borrow())?;
        lockup_schedule_state.is_initialized = true;
        lockup_schedule_state.initializer = *initializer.key;
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
        token_quantity: u64,
        program_id: &Pubkey
    ) -> ProgramResult {

        // get accounts
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;
        let lockup_schedule_state_account = next_account_info(account_info_iter)?;
        let empty_state_account = next_account_info(account_info_iter)?;
        let receiver_account = next_account_info(account_info_iter)?;
        let temp_token_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(next_account_info(account_info_iter)?)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        // check the initializer signed the tx
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // check token program has correct program id
        spl_token::check_program_account(token_program.key)?;

        // check program owns the state accounts
        if lockup_schedule_state_account.owner != program_id {
            return Err(TokenDistributorError::IncorrectOwner.into());
        }
        if empty_state_account.owner != program_id {
            return Err(TokenDistributorError::IncorrectOwner.into());
        }

        // check empty state account has enough lamports to ensure it doesn't get closed by the runtime
        if !rent.is_exempt(empty_state_account.lamports(), empty_state_account.data_len()) {
            return Err(TokenDistributorError::NotRentExempt.into());
        }

        // unpack the lockup schedule state
        let mut lockup_schedule_state = LockupSchedule::unpack(&lockup_schedule_state_account.data.borrow())?;

        // check signer is initializer in lockup schedule
        if *initializer.key != lockup_schedule_state.initializer {
            return Err(TokenDistributorError::IncorrectOwner.into());
        }
        
        // check current time is before lockup start time
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
        let mut lockup_state = Lockup::unpack_unchecked(&empty_state_account.data.borrow())?;

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
        Lockup::pack(lockup_state, &mut empty_state_account.data.borrow_mut())?;
        LockupSchedule::pack(lockup_schedule_state, &mut lockup_schedule_state_account.data.borrow_mut())?;

        Ok(())
    }

    // REDEEM TOKENS 
    fn process_redeem_tokens(
        accounts: &[AccountInfo],
        program_id: &Pubkey
    ) -> ProgramResult {

        // get accounts
        let account_info_iter = &mut accounts.iter();
        let receiving_account = next_account_info(account_info_iter)?;
        let lockup_schedule_state_account = next_account_info(account_info_iter)?;
        let lockup_state_account = next_account_info(account_info_iter)?;
        let lockup_token_account = next_account_info(account_info_iter)?;
        let receiving_token_account = next_account_info(account_info_iter)?;
        let pda_account = next_account_info(account_info_iter)?;
        let token_program = next_account_info(account_info_iter)?;
        let clock_sysvar = next_account_info(account_info_iter)?;
        let clock = &Clock::from_account_info(clock_sysvar)?;
        let current_timestamp = clock.unix_timestamp as u64;

        // check the initializer signed the tx
        if !receiving_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // check token program has correct program id
        spl_token::check_program_account(token_program.key)?;

        // check program owns the state accounts
        if lockup_schedule_state_account.owner != program_id {
            return Err(TokenDistributorError::IncorrectOwner.into());
        }
        if lockup_state_account.owner != program_id {
            return Err(TokenDistributorError::IncorrectOwner.into());
        }

        // unpack lockup state account
        let mut lockup_state = Lockup::unpack_unchecked(&lockup_state_account.data.borrow())?;

        // check signer is same as receiving account in lockup state
        if *receiving_account.key != lockup_state.receiving_account {
            return Err(TokenDistributorError::UnauthorizedAccount.into());
        }

        // check lockup schedule state account is same as written in lockup state
        if *lockup_schedule_state_account.key != lockup_state.lockup_schedule_state {
            return Err(TokenDistributorError::IncorrectSchedule.into());
        }

        // unpack lockup schedule state
        let lockup_schedule_state = LockupSchedule::unpack_unchecked(&lockup_schedule_state_account.data.borrow())?;

        // CALCULATE NO. TOKENS TO REDEEM
        // max no. periods to redeem = total no. periods - periods redeemed
        let periods_to_redeem: u64;
        let max_periods_to_redeem = lockup_schedule_state.number_periods - lockup_state.periods_redeemed;
        if current_timestamp > lockup_schedule_state.start_timestamp {
            // no. periods unlocked not already redeemed = ((current_timestamp - lockup_schedule.start_timestamp) / lockup_schedule.period_duration) - lockup_state.periods_redeemed
            let periods_unlocked = ((current_timestamp - lockup_schedule_state.start_timestamp) / lockup_schedule_state.period_duration) - lockup_state.periods_redeemed;
            // no. periods to redeem = min(max no. periods to redeem, no. periods unlocked)
            periods_to_redeem = cmp::min(max_periods_to_redeem, periods_unlocked);
        } else {
            periods_to_redeem = 0;
        }

        // no. tokens per period = lockup.token_quantity / lockup_schedule.number_periods
        let tokens_per_period = lockup_state.token_quantity / lockup_schedule_state.number_periods;
        // no. tokens to redeem = no. periods to redeem * no. tokens per period
        let tokens_to_redeem = periods_to_redeem * tokens_per_period;

        // INSTRUCTION: send tokens from the lockup token account to receiving token account
        let (pda, bump_seed) = Pubkey::find_program_address(&[b"tokenDistributor"], program_id);
        let transfer_to_receiver_ix = spl_token::instruction::transfer(
            token_program.key, 
            lockup_token_account.key, // src = lockup token account
            receiving_token_account.key, // dst = receiving token account
            &pda, 
            &[&pda], 
            tokens_to_redeem, // quantity 
        )?;
        msg!("Calling the token program to transfer tokens from lockup to receiving account");
        invoke_signed(
            &transfer_to_receiver_ix,
            &[
                token_program.clone(),
                lockup_token_account.clone(),
                receiving_token_account.clone(),
                pda_account.clone(),
            ],
            &[&[&b"tokenDistributor"[..], &[bump_seed]]],
        )?;

        // increment the number of periods redeemed in state
        lockup_state.periods_redeemed += periods_to_redeem;

        // check if all periods have been redeemed
        if lockup_state.periods_redeemed == lockup_schedule_state.number_periods {
            // check lockup token account is empty
            let lockup_token_account_info = TokenAccount::unpack(&lockup_token_account.data.borrow())?;
            let lockup_tokens_remaining = lockup_token_account_info.amount;
            // if any remaining, send to the receiving token account
            if lockup_tokens_remaining != 0 {
                let transfer_remaining_to_receiver_ix = spl_token::instruction::transfer(
                    token_program.key, 
                    lockup_token_account.key, // src = lockup token account
                    receiving_token_account.key, // dst = receiving token account
                    &pda, 
                    &[&pda], 
                    lockup_tokens_remaining, // quantity 
                )?;
                msg!("Calling the token program to transfer tokens from buyer temp to new temp token account...");
                invoke_signed(
                    &transfer_remaining_to_receiver_ix,
                    &[
                        token_program.clone(),
                        lockup_token_account.clone(),
                        receiving_token_account.clone(),
                        pda_account.clone(),
                    ],
                    &[&[&b"tokenDistributor"[..], &[bump_seed]]],
                )?;
        
            }
        }   

        // pack the lockup state accounts (lockup schedule state is unchanged)
        Lockup::pack(lockup_state, &mut lockup_state_account.data.borrow_mut())?;

        Ok(())
    }
}