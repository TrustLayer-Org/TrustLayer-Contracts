# TrustLayer Contracts

Soroban smart contracts for the TrustLayer protocol on the Stellar network. They handle business registration, trust signal recording, and trust score computation and verification.

## What’s in this repo

- **TrustLayer contract** – `register_business`, `record_signal`, `update_trust_score`, `verify_trust_score`
- **Business profiles** – categories, verification tiers, active status, and `BusinessProfile`
- **Signal stats** – per-business signal counts, averages, and `BusinessStats`
- **Tier registry** – query businesses by verification tier or category, and `TierSummary`
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

## Business Signal Stats API

Lightweight aggregates over a business's recorded signals, without recomputing a full trust score:

- `count_signals_for_business` / `has_signals` – how many signals a business has, and whether it has any
- `latest_signal_value` – the most recently recorded signal's value
- `average_signal_value` – mean raw signal value (zero when there are none)
- `signal_type_count` – how many signals of a given type a business has
- `get_business_stats` – aggregate count, average, and presence into a `BusinessStats` view

## Verification Tier Registry API

Query the business registry by verification tier or category, without recomputing profiles one at a time:

- `count_businesses_at_tier` / `list_business_ids_at_tier` – businesses at an exact tier
- `highest_tier` – the highest verification tier among registered businesses
- `list_business_ids_meeting_tier` – businesses at or above a required tier
- `count_businesses_in_category` / `list_business_ids_in_category` – businesses in a given category
- `get_tier_summary` – aggregate count and ids for a tier into a `TierSummary` view

## Contributing

1. Fork the repo and create a branch from `main`.
2. Make changes; keep formatting with `cargo fmt`.
3. Ensure `cargo fmt --all -- --check`, `cargo build`, and `cargo test` pass.
4. Open a pull request to `main`. CI will run fmt, build, and tests.

## License

MIT
