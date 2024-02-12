use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

pub enum CalcInstruction {
    InitState,

    LoadOperand { operand: u64 },

    Add { operand: u64 },

    Sub { operand: u64 },

    Mul { operand: u64 },

    Div { operand: u64 },

    Mod { operand: u64 },
}

#[Derive(BorshDeserialize)]
pub struct Payload {
    operand: u64,
}

impl CalcInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError.InvalidInstructionData)?;

        let payload = Payload::try_from_slice(rest).unwrap();

        Ok(match tag {
            0 => Self::InitState,

            1 => Self::LoadOperand {
                operand: payload.operand,
            },

            2 => Self::Add {
                operand: payload.operand,
            },

            3 => Self::Sub {
                operand: payload.operand,
            },

            4 => Self::Mul {
                operand: payload.operand,
            },

            5 => Self::Div {
                operand: payload.operand,
            },

            6 => Self::Mod {
                operand: payload.operand,
            },

            // _ => return Err(InvalidInstruction.into()),
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    // pub fn unpack_data(input: &[u8]) -> Result<Self, ProgramError> {}
}
