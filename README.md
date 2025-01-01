# Multi-party Computation with a Simplified zk-Rollup System in Rust

This project is a simplified simulation of a zk-rollup system, designed to demonstrate the interaction between users submitting transactions, an operator (Prover) batching these transactions and generating proofs, and a verifier (Mainnet) validating these proofs. It's built with Rust and showcases basic concepts of cryptographic computation and privacy-preserving mechanisms.

## Overview

The system consists of three main components:

- **User Interface**: Allows users to submit transactions to the Prover.
- **Prover (Operator)**: Batches transactions, generates a proof of correctness, and submits this to the Verifier.
- **Verifier (Mainnet)**: Receives proofs from the Prover, issues challenges, and verifies the responses to ensure the integrity of the transactions.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) and Cargo (Rust's package manager) installed on your machine.
- Basic knowledge of Rust programming and terminal/command-line usage.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/luishsr/rust-mpc.git


2. Navigate to the project directory:
    ```bash
    cd zk-rollup-simulation

### Running the Components

1. Start the Verifier:
    ```bash
    cargo run --bin verifier

This launches the Verifier, which listens for proofs from the Prover.

2. Launch the Prover:

    ```bash
    cargo run --bin prover

This starts the Prover, ready to receive transactions from users and interact with the Verifier.

3. Use the User Interface to Submit Transactions:
    ```bash
    cargo run --bin user_tui

Alternatively, you can simulate user transactions using nc.

If manually sending transactions, connect to the Prover using nc and enter transaction data:

      ```bash
      nc localhost 7878  
      {"from":"Alice","to":"Bob","amount":100}  
      {"from":"Charlie","to":"Dave","amount":50}

Press Enter after each transaction and use Ctrl+D (Linux/Mac) or Ctrl+Z followed by Enter (Windows) to end the transmission.
