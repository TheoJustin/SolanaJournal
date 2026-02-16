use anchor_lang::prelude::*;

declare_id!("3BQSZKLPKJ4LkrWgQdZLWGq8PV6kF7NWdH6N8yGFHPZm");

#[program]
pub mod solana_journal {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
