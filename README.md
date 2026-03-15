# TrustLayer Contracts

Soroban smart contracts for the TrustLayer protocol on the Stellar network. They handle business registration, trust signal recording, and trust score computation and verification.

## What’s in this repo

- **TrustLayer contract** – `register_business`, `record_signal`, `update_trust_score`, `verify_trust_score`
- **Tests** – Unit tests in `src/test.rs`
- **CI** – Format check, build, and tests on push/PR to `main`

## Prerequisites

- [Rust](https://rustup.rs/) (stable, with `rustfmt`)
- Optional: [Soroban CLI](https://soroban.stellar.org/docs/develop/developer-tools/soroban-cli) for deployment and local testing

## Setup

```bash
# Clone (or you're already in the repo)
git clone <your-remote>/trustlayer-contracts
cd trustlayer-contracts

# Build
cargo build

# Run tests
cargo test

# Check formatting
cargo fmt --all -- --check
```

## Project layout

- `src/lib.rs` – Contract types and implementation
- `src/test.rs` – Tests
- `Cargo.toml` – Dependencies and profile (release uses `opt-level = "z"` for contracts)

## Contributing

1. Fork the repo and create a branch from `main`.
2. Make changes; keep formatting with `cargo fmt`.
3. Ensure `cargo fmt --all -- --check`, `cargo build`, and `cargo test` pass.
4. Open a pull request to `main`. CI will run fmt, build, and tests.

## License

MIT
