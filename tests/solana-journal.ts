import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import { expect } from 'chai';
import { SolanaJournal } from '../target/types/solana_journal';

describe('solana-journal', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaJournal as Program<SolanaJournal>;
  const wallet = provider.wallet as anchor.Wallet;

  const title = 'My First Entry';
  const message = 'Solana Rust is finally making sense!';

  const [journalEntryPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from('journal_entry'),
      wallet.publicKey.toBuffer(),
      Buffer.from(title),
    ],
    program.programId,
  );

  it('Can create a new journal entry', async () => {
    const tx = await program.methods
      .createJournalEntry(title, message)
      .accounts({ payer: wallet.publicKey })
      .rpc();

    const entry = await program.account.journalEntry.fetch(journalEntryPda);
    expect(entry.title).to.equal(title);
    expect(entry.message).to.equal(message);
    expect(entry.owner.toBase58()).to.equal(wallet.publicKey.toBase58());
  });

  it('Can fetch all journal entries', async () => {
    const entries = await program.account.journalEntry.all();

    expect(entries.length).to.be.greaterThan(0);
    console.log(`Successfully fetched ${entries.length} journal entries!`);

    entries.forEach((entry) => {
      console.log(
        `Title: ${entry.account.title}, Message: ${entry.account.message}`,
      );
    });
  });

  it('Can update a journal entry', async () => {
    const newMessage = 'This update is replacing my old text!';

    const tx = await program.methods
      .updateJournalEntry(title, newMessage)
      .accounts({ payer: wallet.publicKey })
      .rpc();

    const entry = await program.account.journalEntry.fetch(journalEntryPda);
    expect(entry.title).to.equal(title);
    expect(entry.message).to.equal(newMessage);
  });

  it('Can delete a journal entry', async () => {
    const tx = await program.methods
      .deleteJournalEntry(title)
      .accounts({ payer: wallet.publicKey })
      .rpc();

    try {
      await program.account.journalEntry.fetch(journalEntryPda);

      expect.fail('The account should have been completely deleted.');
    } catch (error) {
      expect(error.message).to.include('Account does not exist');
    }
  });
});
