# TrustLayer Contracts

Soroban smart contracts for the TrustLayer protocol on the Stellar network. They handle business registration, trust signal recording, and trust score computation and verification.

## What’s in this repo

- **TrustLayer contract** – `register_business`, `record_signal`, `update_trust_score`, `verify_trust_score`
- **Business profiles** – categories, verification tiers, active status, and `BusinessProfile`
- **Signal stats** – per-business signal counts, averages, and `BusinessStats`
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

## Business Profile API

Beyond scoring, the contract stores lightweight profile metadata per business:

- `set_category` / `get_category` – business category (defaults to `none`)
- `set_verification_tier` / `get_verification_tier` – tier (defaults to `0`)
- `bump_tier` / `downgrade_tier` – adjust the tier by one
- `deactivate_business` / `reactivate_business` / `is_active` – active status
- `is_verified` / `is_active_and_verified` / `meets_tier` – status checks
- `set_profile` / `get_profile` – set or read the full `BusinessProfile`
- `register_verified_business` – register and set a tier in one call
- `get_business` / `count_businesses` / `count_active_businesses` – registry queries

## Contributing

1. Fork the repo and create a branch from `main`.
2. Make changes; keep formatting with `cargo fmt`.
3. Ensure `cargo fmt --all -- --check`, `cargo build`, and `cargo test` pass.
4. Open a pull request to `main`. CI will run fmt, build, and tests.

## License

MIT
