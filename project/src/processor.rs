use solana_program::{
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey
}

use crate::{instruction::CalcInstruction}

pub struct Processor;
impl Processor {
    pub fn process(
        program_id:         &Pubkey,
        accounts:           &[AccountInfo],
        instruction_data:   &[u8],
    ) -> ProgramResult {

        let instruction;

        match instruction {

            CalcInstruction::InitCalculatorState { output } => {.

                msg!("Init of Output Account");

                return Self::process_init_output(accounts, output, program_id);
            }

            CalcInstruction::AddOperation { operandA, operandB } => {

                msg!("OUTPUT: {}", output);

                return Self::process_add_operation(accounts, operandA, operandB, program_id);
            }

            CalcInstruction::SubOperation { operandA, operandB } => {

                msg!("OUTPUT: {}", output);

                return Self::process_sub_operation(accounts, operandA, operandB, program_id);
            }

            CalcInstruction::MulOperation { operandA, operandB } => {

                msg!("OUTPUT: {}", output);

                return Self::process_sub_operation(accounts, operandA, operandB, program_id);
            }

            CalcInstruction::DivOperation { operandA, operandB } => {

                msg!("OUTPUT: {}", output);

                return Self::process_sub_operation(accounts, operandA, operandB, program_id);
            }

        }
    }

    fn process_init_output(
        accounts:   &[AccountInfo],
        output:     u64,
        program_id: &Pubkey
    ) -> ProgramResult {

    }

    fn process_add_operation(
        accounts:   &[AccountInfo],
        operandA:   u64,
        operandB:   u64,
        program_id: &Pubkey
    ) -> ProgramResult {

        let result = operandA + operandB;
        
    }

    fn process_sub_operation(
        accounts:   &[AccountInfo],
        operandA:   u64,
        operandB:   u64,
        program_id: &Pubkey
    ) -> ProgramResult {

        let result = operandA - operandB;
        
    }

    fn process_mul_operation(
        accounts:   &[AccountInfo],
        operandA:   u64,
        operandB:   u64,
        program_id: &Pubkey
    ) -> ProgramResult {
        
        let result = operandA * operandB;

    }

    fn process_div_operation(
        accounts:   &[AccountInfo],
        operandA:   u64,
        operandB:   u64,
        program_id: &Pubkey
    ) -> ProgramResult {
        
        let result = operandA / operandB;

    }

    fn process_mod_operation(
        accounts:   &[AccountInfo],
        operandA:   u64,
        operandB:   u64,
        program_id: &Pubkey
    ) -> ProgramResult {
        
        let result = operandA % operandB;

    }
}