use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct JournalEntry {
    #[max_len(50)]
    pub title: String,
    #[max_len(500)]
    pub message: String,
    pub bump: u8,
    pub owner: Pubkey,
}

impl JournalEntry {
    pub const SEED_PREFIX: &'static [u8; 13] = b"journal_entry";

    pub fn add_entry(&mut self, title: String, message: String, owner: Pubkey){
        self.title = title;
        self.message = message;
        self.owner = owner;
    }

    pub fn update_entry(&mut self, message: String){
        self.message = message;
    }
}