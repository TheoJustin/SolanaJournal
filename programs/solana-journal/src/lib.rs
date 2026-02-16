use anchor_lang::prelude::*;

use instructions::*;

pub mod instructions;
pub mod state;

declare_id!("3BQSZKLPKJ4LkrWgQdZLWGq8PV6kF7NWdH6N8yGFHPZm");


#[program]
pub mod solana_journal {
    use super::*;

    pub fn create_journal_entry(ctx: Context<CreateJournalEntry>, title: String, message: String) -> Result<()> {
        add_entry::create_journal_entry(ctx, title, message)
    }

    pub fn update_journal_entry(ctx: Context<UpdateJournalEntry>, title: String, message: String) -> Result<()> {
        update_entry::update_journal_entry(ctx, title, message)
    }

    pub fn delete_journal_entry(ctx: Context<DeleteJournalEntry>, title: String) -> Result<()> {
        delete_entry::delete_journal_entry(ctx, title)
    }
}