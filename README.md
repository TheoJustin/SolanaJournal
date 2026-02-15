# Solana Web3 Journal 📝

A minimalist decentralized application that allows users to create, read, update, and delete personal journal entries directly on the Solana blockchain.

## 📖 Overview

This project serves as an introductory CRUD application built with the Anchor framework. It demonstrates the fundamental mechanics of allocating space on-chain and securely associating data with a user's wallet without the overhead of complex tokenomics.

### Core Features

* **Write**: Users can publish a new journal entry with a title and a message.
* **Read**: Users can fetch and display all their on-chain entries.
* **Edit**: Users can modify the text of an existing entry.
* **Delete**: Users can close the account to delete the entry and reclaim their storage rent.

## 🛠 Tech Stack

* **Smart Contract**: Rust, Solana, Anchor Framework
* **Environment**: VSCode (recommended with `rust-analyzer`)

## ⚙️ Prerequisites

Before running this project, ensure you have the following installed on your machine:

* [Rust](https://www.rust-lang.org/tools/install)
* [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
* [Anchor CLI](https://www.anchor-lang.com/docs/installation)
* [Node.js](https://nodejs.org/) & [Yarn](https://yarnpkg.com/)

## 🚀 Getting Started

### 1. Clone the repository and install dependencies

```bash
git clone <your-repo-url>
cd solana-journal
yarn install
```

### 2. Build the Anchor program

This will compile the Rust smart contract and generate the necessary IDL files.

```bash
anchor build
```

### 3. Run the test suite

Boot up a local Solana test validator and execute the TypeScript test suite against the compiled program to verify the CRUD logic.

```bash
anchor test
```

## 📂 Project Structure

```
solana-journal/
├── programs/
│   └── solana-journal/
│       └── src/
│           └── lib.rs          # Core smart contract logic
├── tests/
│   └── solana-journal.ts       # TypeScript testing suite
├── Anchor.toml                 # Anchor workspace configuration
└── README.md
```

* `programs/solana-journal/src/lib.rs` - The core smart contract logic written in Rust.
* `tests/solana-journal.ts` - The TypeScript testing suite simulating the creation, editing, and deletion of journal entries.
* `Anchor.toml` - The Anchor workspace configuration and network settings.

## 📝 Usage

### Creating a Journal Entry

```typescript
await program.methods
  .createEntry(title, message)
  .accounts({
    entry: entryAccount.publicKey,
    owner: wallet.publicKey,
  })
  .rpc();
```

### Reading Journal Entries

```typescript
const entries = await program.account.journalEntry.all([
  {
    memcmp: {
      offset: 8,
      bytes: wallet.publicKey.toBase58(),
    }
  }
]);
```

### Updating a Journal Entry

```typescript
await program.methods
  .updateEntry(newTitle, newMessage)
  .accounts({
    entry: entryAccount.publicKey,
    owner: wallet.publicKey,
  })
  .rpc();
```

### Deleting a Journal Entry

```typescript
await program.methods
  .deleteEntry()
  .accounts({
    entry: entryAccount.publicKey,
    owner: wallet.publicKey,
  })
  .rpc();
```

## 🔧 Development

### Local Development

1. Start a local Solana test validator:
```bash
solana-test-validator
```

2. Deploy the program to the local network:
```bash
anchor deploy
```

3. Run tests:
```bash
anchor test --skip-local-validator
```

### Deploying to Devnet

1. Configure Solana CLI to use devnet:
```bash
solana config set --url devnet
```

2. Airdrop SOL for deployment costs:
```bash
solana airdrop 2
```

3. Deploy to devnet:
```bash
anchor deploy --provider.cluster devnet
```

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the issues page.

## 📄 License

This project is [MIT](LICENSE) licensed.

## 🙏 Acknowledgments

* [Anchor Framework](https://www.anchor-lang.com/) - The framework that powers this dApp
* [Solana](https://solana.com/) - The blockchain platform
* The Solana developer community for excellent documentation and support

---

Built with ❤️ on Solana