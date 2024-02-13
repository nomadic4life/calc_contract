use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct OutputAccount {
    // outputBuffer
    pub output: i64,
}
