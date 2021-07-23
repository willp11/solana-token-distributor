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

use crate::{instruction::TokenDistributorInstruction}

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