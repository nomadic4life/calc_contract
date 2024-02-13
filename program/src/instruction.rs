use borsh::BorshDeserialize;
use solana_program::msg;
use solana_program::program_error::ProgramError;

pub enum CalcInstruction {
    InitState,

    LoadOperand { operand: i64 },

    Add { operand: i64 },

    Sub { operand: i64 },

    Mul { operand: i64 },

    Div { operand: i64 },

    Mod { operand: i64 },
}

#[derive(BorshDeserialize)]
pub struct Payload {
    operand: i64,
}

impl CalcInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        msg!("before payload");
        msg!("len: {}, {}", rest.len(), input.len());
        msg!(
            "byte: {}, {}, {}, {}, {}, {}, {}, {}, {}",
            input[0],
            input[1],
            input[2],
            input[3],
            input[4],
            input[5],
            input[6],
            input[7],
            input[8]
        );

        let payload = Payload::try_from_slice(rest).unwrap();

        msg!("serialized payload: {}", payload.operand);

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
