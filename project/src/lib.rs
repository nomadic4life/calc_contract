pub mod instruction;
pub mod processor;
pub mod state;

use crate::processor::Processor;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    return Processor::process(program_id, accounts, instruction_data);
}

// Create a Solana contract with a boilerplate hello world code that can be found here.
// Instead of processing just one instruction, create functions to calculate the sum and difference of two numbers.
// The corresponding functions should be called based on user’s input.
// Final result should be stored in an account created by the program.

// Functionality:
// Solana contract with boilerplate hello world is created
// Function calculates the sum of two numbers chosen by user
// Function calculates the difference of two numbers chosen by user
// Final results are stored in an account created by the program

// Explanation:
// Demo and code read-aloud is submitted
// Demo and code read-aloud is complete (all steps explained)
// Demo and code read-aloud is clear and understandable

// github -> http://github.com/nomadic4life/calc_contract
// video walkthrough   ::   youtube or loom ->
//  -> Please record and upload a video walkthrough of your code with loom, explaining your thought processes and decisions. Please make sure the video is publicly accessible so our reviewers can take a look.
// Transaction ID / Application ID
//  -> Depending on the project prompt, you may be asked to conduct a transaction or deploy an application. Please put in the appropriate identifier for this project.

// source:
// https://academy.metacrafters.io/content/solana-intermediate/solana-program/assessment/project

// https://academy.metacrafters.io/content/solana-intern/tokens-nfts
