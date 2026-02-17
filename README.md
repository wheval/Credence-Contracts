# Credence Contracts

Soroban smart contracts for the Credence economic trust protocol. This workspace holds the identity bond contract (lock USDC, track duration, slashing hooks).

## About

Part of [Credence](../README.md). Contracts run on the Stellar network via Soroban. The bond contract is the source of truth for staked amounts and is consumed by the backend reputation engine.

## Prerequisites

- Rust 1.84+ (with `wasm32-unknown-unknown`: `rustup target add wasm32-unknown-unknown`)
- [Soroban CLI](https://developers.stellar.org/docs/smart-contracts/getting-started/setup) (`cargo install soroban-cli`)

## Setup

From the repo root:

```bash
cd credence-contracts
cargo build
```

For Soroban (WASM) build:

```bash
cargo build --target wasm32-unknown-unknown --release -p credence_bond
```

## Tests

```bash
cargo test -p credence_bond
```

## Project layout

- `contracts/credence_bond/` — Identity bond contract
  - `create_bond()` — lock USDC (stub: stores amount and duration)
  - `get_identity_state()` — return current bond for this instance

A full implementation would add:

- Token transfer (USDC) on `create_bond` / `increase_bond` / `withdraw_bond`
- `slash_bond()` with governance checks
- `add_attestation()` / `revoke_attestation()`
- Per-identity storage (e.g. by `Address` key) instead of a single bond

## Deploy (Soroban CLI)

Configure network and deploy:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/credence_bond.wasm \
  --source <SECRET_KEY> \
  --network <NETWORK>
```

See [Stellar Soroban docs](https://developers.stellar.org/docs/smart-contracts) for auth and network setup.
