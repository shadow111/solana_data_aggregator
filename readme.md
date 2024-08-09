

# Solana Data Aggregator

## Overview

The Solana Data Aggregator is a Rust-based application designed to retrieve, process, and expose transaction and account data from the Solana blockchain. The application is capable of real-time monitoring, data aggregation, and provides a RESTful API for querying the collected data.

## Features

- **Real-Time Data Retrieval:** Continuously monitors the Solana blockchain for new transactions and account updates.
- **Data Processing:** Efficiently parses and organizes transaction records, extracting relevant details for further analysis.
- **Optional Data Storage:** Supports both in-memory and persistent storage solutions for scalability and reliability.
- **RESTful API:** Provides an API layer for external applications to query transaction history, account details, and more.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.56+)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- A running Solana node on devnet or testnet ([Helius.dev](https://www.helius.dev/) or other providers)

### Clone the Repository

```bash
git clone https://github.com/shadow111/solana-data-aggregator.git
cd solana-data-aggregator
```

### Build the Project

```bash
cargo build --release
```

### Run the Application

```bash
cargo run --release
```

### Configuration

Create a configuration file in the project root (e.g., `config.toml`):

```toml
solana_rpc_url = "https://api.devnet.solana.com"
solana_ws_url = "wss://api.devnet.solana.com"
api_bind_address = "127.0.0.1:8080"
database_url = "postgres://user:password@localhost/db"
transaction_signature = "your_transaction_signature_here"
account_pubkey = "your_account_pubkey_here"
port = "8080"
```

## Usage

### REST API

Once the application is running, the RESTful API can be accessed via the configured bind address. The following endpoints are available:

- **GET /api/transaction/signature/:signature:** Retrieve transaction by signature.
- **GET /accounts/:pubkey:** Get details for a specific account.
- **GET /api/transaction/slot/:slot:** Retrieve block data for a given slot.

### Example Requests

```bash
curl http://127.0.0.1:8000/api/transaction/signature/:signature
curl http://127.0.0.1:8000/api/account/:pubkey
curl http://127.0.0.1:8000/api/transaction/slot/:slot
```

## Testing

Unit tests are provided to ensure the reliability and correctness of the application. To run the tests, use the following command:

```bash
cargo test
```
