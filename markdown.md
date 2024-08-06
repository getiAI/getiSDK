# Project: Smart Contracts Using Stylus SDK

## Introduction

This project consists of two Rust-based smart contracts developed using the Stylus SDK. The contracts demonstrate basic functionalities such as persistent storage management and token transactions on the Ethereum blockchain. The Stylus SDK offers an efficient and secure framework for developing Ethereum smart contracts in Rust.

## Product Overview

### SimpleStorage Contract

**Purpose**: 
The `SimpleStorage` contract is designed to store and retrieve a single `uint256` value. It serves as an introductory example of using the Stylus SDK for managing persistent storage in smart contracts.

**Key Features**:
- **Persistent Storage**: Stores a single `uint256` value that persists across transactions.
- **Simple Interface**: Provides straightforward methods for setting and retrieving the stored value.

**Functions**:
- `set_value(&mut self, value: u64)`: This method sets the value in storage.
- `get_value(&self) -> u64`: This method retrieves the current value from storage.

**Entry Point**:
- `user_main(input: Vec<u8>) -> Vec<u8>`:
  - If the input is empty, the function retrieves and returns the current stored value.
  - If the input is not empty, it interprets the input as a `u64` value, sets the storage to this value, and returns an empty vector.

**Use Cases**:
- **Data Storage**: Suitable for applications requiring simple storage mechanisms.
- **Example Contract**: Serves as a learning tool for developers new to the Stylus SDK.

### SimpleToken Contract

**Purpose**:
The `SimpleToken` contract implements a basic ERC-20 token. It demonstrates token creation, balance tracking, and token transfers using the Stylus SDK.

**Key Features**:
- **Token Management**: Manages a token with a name, symbol, and total supply.
- **Balance Tracking**: Keeps track of token balances for each address.
- **Token Transfers**: Allows transferring tokens between addresses.

**Functions**:
- `new(name: String, symbol: String, total_supply: u64)`: Initializes a new token contract with the specified name, symbol, and total supply.
- `balance_of(&self, owner: address) -> u64`: Returns the token balance of the specified address.
- `transfer(&mut self, to: address, amount: u64) -> bool`: Transfers the specified amount of tokens to the given address.

**Entry Point**:
- `user_main(input: Vec<u8>) -> Vec<u8>`: Placeholder for entry point logic, which can be extended to handle specific interactions with the contract.

**Use Cases**:
- **Token Creation**: Useful for creating and managing tokens in decentralized applications.
- **Financial Transactions**: Can be integrated into applications requiring token-based transactions.
- **Learning Tool**: Provides an example for developers to understand token management using the Stylus SDK.

## Technical Details

### Stylus SDK

The Stylus SDK is built on top of Alloy, a collection of crates that empower the Rust Ethereum ecosystem. It uses the same Rust primitives for Ethereum types, ensuring compatibility with existing Rust libraries.

**Key Features**:
- **No Standard Library**: Stylus programs use `#![no_std]` to avoid including the Rust standard library, keeping the code small and efficient.
- **Storage Management**: Provides macros like `sol_storage!` for defining storage structures that map directly to Ethereum's storage trie.
- **Entry Points**: Uses the `#[entrypoint]` macro to define contract entry points.

### Development and Deployment

**Development Environment**:
- **Rust**: The programming language used for developing the smart contracts.
- **Cargo**: The Rust package manager and build system.

**Compilation**:
- **WASM Target**: The Stylus VM supports Rust's `wasm32-unknown-unknown` target.

**Deployment**:
- Contracts can be deployed to the Ethereum blockchain using the tools provided by the Stylus SDK.

## Conclusion

The `SimpleStorage` and `SimpleToken` contracts demonstrate fundamental blockchain operations such as data storage and token management using the Stylus SDK. These contracts provide a solid foundation for developing more complex decentralized applications and serve as valuable learning tools for developers.
