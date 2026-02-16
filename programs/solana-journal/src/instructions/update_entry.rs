use crate::state::JournalEntry;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateJournalEntry<'info> {
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
    )]
    pub journal_entry: Account<'info, JournalEntry>,
}

pub fn update_journal_entry(ctx: Context<UpdateJournalEntry>, _title:String, message: String) -> Result<()> {
    let journal_entry = &mut ctx.accounts.journal_entry;

    journal_entry.update_entry(message);

    Ok(())
}