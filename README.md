# Rust-Bitcoin-wallet-Generator

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.60.0-orange.svg)](https://www.rust-lang.org/)
[![Bitcoin](https://img.shields.io/badge/Bitcoin-BDK-yellow.svg)](https://github.com/bitcoindevkit/bdk)

## Overview

This is a Rust example showcasing the use of the `bdk` (Bitcoin Development Kit) library to build a simple Bitcoin wallet. The example demonstrates key functionalities such as syncing with the blockchain, checking balances, generating new addresses, and creating and broadcasting transactions.

## Features

- **Blockchain Integration**: Utilizes the Electrum blockchain through the `bdk` library.
- **Wallet Creation**: Demonstrates how to create a Bitcoin wallet with specific descriptors.
- **Transaction Building**: Illustrates the process of building and signing transactions.
- **Testnet Integration**: Configured to work on the Bitcoin testnet.

## Prerequisites

- Rust 1.60.0 or later
- [bdk](https://github.com/bitcoindevkit/bdk) library
- [Electrum Server](https://electrum.org/) (In this example, we use Blockstream's Electrum server)

## Getting Started

### Installation

```bash
git clone https://github.com/[ayoseun]/rust-bdk-wallet-example.git
cd rust-bdk-wallet-example
cargo build --release
```

### Usage

```bash
cargo run
```
Follow the on-screen prompts to see the wallet in action.

## Example Workflow
Sync with Blockchain: Connects to the Electrum server and synchronizes the wallet with the testnet blockchain.

- Check Balance: Displays the wallet balance based on the synced data.

- Generate New Address: Creates a new Bitcoin address for receiving funds.

- Build and Sign Transaction: Builds a transaction to drain the wallet to a specified address and signs it.

- Broadcast Transaction: Broadcasts the signed transaction to the testnet.

- Explore Transaction: Provides a link to explore the transaction on a block explorer.

### Contributing
Feel free to contribute by forking the repository, creating pull requests, or opening issues.

### License
This project is licensed under the MIT License - see the LICENSE file for details.

### Acknowledgments
bdk developers for providing a powerful Rust library for Bitcoin applications.
Blockstream's Electrum server for blockchain interaction.

### Contact
For any questions or concerns, feel free to contact [ayoseun].