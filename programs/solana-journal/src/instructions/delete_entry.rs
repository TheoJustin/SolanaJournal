use crate::state::JournalEntry;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteJournalEntry<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            JournalEntry::SEED_PREFIX,
            payer.key().as_ref(),
            title.as_bytes(),
        ],
        bump = journal_entry.bump,
        close = payer,
    )]
    pub journal_entry: Account<'info, JournalEntry>,
}

pub fn delete_journal_entry(ctx: Context<UpdateJournalEntry>, message: String) -> Result<()> {
    Ok(())
}