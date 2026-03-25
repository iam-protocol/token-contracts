#![deny(clippy::all)]

use anchor_lang::prelude::*;

declare_id!("Bp1B4Azj7AVjvg855Xd7AiPrJFpSxMZMqRSbpUcXaSRe");

#[program]
pub mod iam_token {
    use super::*;

    /// Initialize the IAM token mint with Confidential Balances extension.
    /// Implementation in Phase 7.
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
