use crate::state::JournalEntry;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateJournalEntry<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        init,
        space = 8 + JournalEntry::INIT_SPACE,
        payer = payer,
        seeds = [
            JournalEntry::SEED_PREFIX,
            payer.key().as_ref(),
            title.as_bytes(),
        ],
        bump,
    )]
    pub journal_entry: Account<'info, JournalEntry>,
    pub system_program: Program<'info, System>,
}

pub fn create_journal_entry(ctx: Context<CreateJournalEntry>, title: String, message: String) -> Result<()> {
    let journal_entry = &mut ctx.accounts.journal_entry;

    journal_entry.add_entry(title, message, ctx.accounts.payer.key());

    journal_entry.bump = ctx.bumps.journal_entry;

    Ok(())
}