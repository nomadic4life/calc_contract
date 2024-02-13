use crate::instruction::CalcInstruction;
use crate::state::OutputAccount;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    // borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::{self},
    sysvar::rent::Rent,
    sysvar::Sysvar,
};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CalcInstruction::unpack(instruction_data).unwrap();

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
        let account_iter = &mut accounts.iter();

        let user_account = next_account_info(account_iter)?;
        let user_calc_state_account = next_account_info(account_iter)?;
        let system_account = next_account_info(account_iter)?;

        msg!("is write: {}", user_calc_state_account.is_writable);

        assert!(user_account.is_signer);
        assert!(user_calc_state_account.is_writable);

        let seed = "output_buffer";

        let (pda, bump) = Pubkey::find_program_address(
            &[user_account.key.as_ref(), &seed.as_bytes().as_ref()],
            program_id,
        );

        msg!("{} === {} ??", pda, user_calc_state_account.key);

        let rent = Rent::get()?.minimum_balance(8);

        invoke_signed(
            &system_instruction::create_account(
                user_account.key,
                user_calc_state_account.key,
                rent,
                8,
                program_id,
            ),
            &[
                user_account.clone(),
                user_calc_state_account.clone(),
                system_account.clone(),
            ],
            &[&[
                &user_account.key.as_ref(),
                &seed.as_bytes().as_ref(),
                &[bump],
            ]],
        )?;

        let mut output_account: OutputAccount =
            OutputAccount::try_from_slice(&user_calc_state_account.data.borrow())?;

        output_account.output = 0;

        msg!("output account initated, value set to 0");

        output_account.serialize(&mut &mut user_calc_state_account.data.borrow_mut()[..])?;

        return Ok(());
    }

    fn process_load_operand(
        accounts: &[AccountInfo],
        operand: i64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();

        let user_account = next_account_info(account_iter)?;
        let user_calc_state_account = next_account_info(account_iter)?;

        assert!(user_account.is_signer);
        assert!(user_calc_state_account.is_writable);

        let seed = "output_buffer";

        let (pda, _) = Pubkey::find_program_address(
            &[user_account.key.as_ref(), &seed.as_bytes().as_ref()],
            program_id,
        );

        if user_calc_state_account.key != &pda {
            // can handle this differently if the owner key of pda is stored as state
            // and so we don't have to derive with find_program_address, can save gas.
            return Err(ProgramError::InvalidAccountData);
        }

        let mut output_account: OutputAccount =
            OutputAccount::try_from_slice(&mut user_calc_state_account.data.borrow())?;

        msg!("LOAD: {}", operand);

        output_account.output = operand;

        msg!("OUTPUT: {}", output_account.output);

        output_account.serialize(&mut &mut user_calc_state_account.data.borrow_mut()[..])?;

        return Ok(());
    }

    fn process_add_operation(
        accounts: &[AccountInfo],
        operand: i64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();

        let user_account = next_account_info(account_iter)?;
        let user_calc_state_account = next_account_info(account_iter)?;

        assert!(user_account.is_signer);
        assert!(user_calc_state_account.is_writable);

        let seed = "output_buffer";

        let (pda, _) = Pubkey::find_program_address(
            &[user_account.key.as_ref(), &seed.as_bytes().as_ref()],
            program_id,
        );

        if user_calc_state_account.key != &pda {
            // can handle this differently if the owner key of pda is stored as state
            // and so we don't have to derive with find_program_address, can save gas.
            return Err(ProgramError::InvalidAccountData);
        }

        let mut output_account: OutputAccount =
            OutputAccount::try_from_slice(&mut user_calc_state_account.data.borrow())?;

        msg!("ADD: {} + {}", output_account.output, operand);

        output_account.output += operand;

        msg!("OUTPUT: {}", output_account.output);

        output_account.serialize(&mut &mut user_calc_state_account.data.borrow_mut()[..])?;

        return Ok(());
    }

    fn process_sub_operation(
        accounts: &[AccountInfo],
        operand: i64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();

        let user_account = next_account_info(account_iter)?;
        let user_calc_state_account = next_account_info(account_iter)?;

        assert!(user_account.is_signer);
        assert!(user_calc_state_account.is_writable);

        let seed = "output_buffer";

        let (pda, _) = Pubkey::find_program_address(
            &[user_account.key.as_ref(), &seed.as_bytes().as_ref()],
            program_id,
        );

        if user_calc_state_account.key != &pda {
            // can handle this differently if the owner key of pda is stored as state
            // and so we don't have to derive with find_program_address, can save gas.
            return Err(ProgramError::InvalidAccountData);
        }

        let mut output_account: OutputAccount =
            OutputAccount::try_from_slice(&mut user_calc_state_account.data.borrow())?;

        msg!("SUB: {} - {}", output_account.output, operand);

        output_account.output -= operand;

        msg!("OUTPUT: {}", output_account.output);

        output_account.serialize(&mut &mut user_calc_state_account.data.borrow_mut()[..])?;

        return Ok(());
    }

    fn process_mul_operation(
        accounts: &[AccountInfo],
        operand: i64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();

        let user_account = next_account_info(account_iter)?;
        let user_calc_state_account = next_account_info(account_iter)?;

        assert!(user_account.is_signer);
        assert!(user_calc_state_account.is_writable);

        let seed = "output_buffer";

        let (pda, _) = Pubkey::find_program_address(
            &[user_account.key.as_ref(), &seed.as_bytes().as_ref()],
            program_id,
        );

        if user_calc_state_account.key != &pda {
            // can handle this differently if the owner key of pda is stored as state
            // and so we don't have to derive with find_program_address, can save gas.
            return Err(ProgramError::InvalidAccountData);
        }

        let mut output_account: OutputAccount =
            OutputAccount::try_from_slice(&mut user_calc_state_account.data.borrow())?;

        msg!("MUL: {} * {}", output_account.output, operand);

        output_account.output *= operand;

        msg!("OUTPUT: {}", output_account.output);

        output_account.serialize(&mut &mut user_calc_state_account.data.borrow_mut()[..])?;

        return Ok(());
    }

    fn process_div_operation(
        accounts: &[AccountInfo],
        operand: i64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();

        let user_account = next_account_info(account_iter)?;
        let user_calc_state_account = next_account_info(account_iter)?;

        assert!(user_account.is_signer);
        assert!(user_calc_state_account.is_writable);

        let seed = "output_buffer";
        let (pda, _) = Pubkey::find_program_address(
            &[user_account.key.as_ref(), &seed.as_bytes().as_ref()],
            program_id,
        );

        if user_calc_state_account.key != &pda {
            // can handle this differently if the owner key of pda is stored as state
            // and so we don't have to derive with find_program_address, can save gas.
            return Err(ProgramError::InvalidAccountData);
        }

        let mut output_account: OutputAccount =
            OutputAccount::try_from_slice(&mut user_calc_state_account.data.borrow())?;

        msg!("DIV: {} / {}", output_account.output, operand);

        output_account.output /= operand;

        msg!("OUTPUT: {}", output_account.output);

        output_account.serialize(&mut &mut user_calc_state_account.data.borrow_mut()[..])?;

        return Ok(());
    }

    fn process_mod_operation(
        accounts: &[AccountInfo],
        operand: i64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_iter = &mut accounts.iter();

        let user_account = next_account_info(account_iter)?;
        let user_calc_state_account = next_account_info(account_iter)?;

        assert!(user_account.is_signer);
        assert!(user_calc_state_account.is_writable);

        let seed = "output_buffer";

        let (pda, _) = Pubkey::find_program_address(
            &[user_account.key.as_ref(), &seed.as_bytes().as_ref()],
            program_id,
        );

        if user_calc_state_account.key != &pda {
            // can handle this differently if the owner key of pda is stored as state
            // and so we don't have to derive with find_program_address, can save gas.
            return Err(ProgramError::InvalidAccountData);
        }

        let mut output_account: OutputAccount =
            OutputAccount::try_from_slice(&mut user_calc_state_account.data.borrow())?;

        msg!("MOD: {} % {}", output_account.output, operand);

        output_account.output %= operand;

        msg!("OUTPUT: {}", output_account.output);

        output_account.serialize(&mut &mut user_calc_state_account.data.borrow_mut()[..])?;

        return Ok(());
    }
}
