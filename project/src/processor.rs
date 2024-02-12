use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};

// use crate::{instruction::CalcInstruction};
// use instruction::CalcInstruction;

use crate::instruction::CalcInstruction;

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction;

        match instruction {
            CalcInstruction::InitState => {
                msg!("Init Output Account");

                return Self::process_init_state(accounts, program_id);
            }

            CalcInstruction::LoadOperand { operand } => {
                msg!("Clear and load data into state buffer: {}", operand);

                return Self::process_load_operand(accounts, operand, program_id);
            }

            CalcInstruction::Add { operand } => {
                msg!("ADD: {}", operand);

                return Self::process_add_operation(accounts, operand, program_id);
            }

            CalcInstruction::Sub { operand } => {
                msg!("SUB: {}", operand);

                return Self::process_sub_operation(accounts, operand, program_id);
            }

            CalcInstruction::Mul { operand } => {
                msg!("MUL: {}", operand);

                return Self::process_mul_operation(accounts, operand, program_id);
            }

            CalcInstruction::Div { operand } => {
                msg!("DIV: {}", operand);

                return Self::process_div_operation(accounts, operand, program_id);
            }

            CalcInstruction::Mod { operand } => {
                msg!("MOD: {}", operand);

                return Self::process_mod_operation(accounts, operand, program_id);
            }
        }
    }

    fn process_init_state(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        return Ok(());
    }

    fn process_load_operand(
        accounts: &[AccountInfo],
        operand: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        return Ok(());
    }

    fn process_add_operation(
        accounts: &[AccountInfo],
        operand: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        return Ok(());
    }

    fn process_sub_operation(
        accounts: &[AccountInfo],
        operand: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        return Ok(());
    }

    fn process_mul_operation(
        accounts: &[AccountInfo],
        operand: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        return Ok(());
    }

    fn process_div_operation(
        accounts: &[AccountInfo],
        operand: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        return Ok(());
    }

    fn process_mod_operation(
        accounts: &[AccountInfo],
        operand: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        return Ok(());
    }

    fn store(output: u64, program_id: &Pubkey) -> () {
        let (pda, _bump) = Pubkey::find_program_addres(&[b"output_buffer"], program_id);

        return Ok(());
    }
}
