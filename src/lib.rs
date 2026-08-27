#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, Address, Env, Map,
    String, Symbol, SymbolStr, TryFromVal, Vec,
};

const AUTHORITY_KEY: &str = "authority";

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 10,
    AlreadyInitialized = 11,
    Unauthorized = 12,
}

/// Stable validation failures for the versioned signal schema.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum SignalError {
    /// The signal symbol is empty or is not in the supported schema.
    UnsupportedSignalType = 1,
    /// The symbol exceeds the schema's bounded metadata length.
    SignalTypeTooLong = 2,
    /// The signal value is outside the inclusive schema range.
    SignalValueOutOfBounds = 3,
}

/// Versioned schema metadata exposed to off-chain clients.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalSchema {
    pub version: u32,
    pub min_value: i128,
    pub max_value: i128,
    pub max_type_len: u32,
    pub allowed_types: Vec<Symbol>,
}

const SIGNAL_SCHEMA_VERSION: u32 = 1;
const SIGNAL_MIN_VALUE: i128 = 0;
const SIGNAL_MAX_VALUE: i128 = 1_000_000;
const SIGNAL_MAX_TYPE_LEN: u32 = 16;

const MAX_WALLET_LENGTH: usize = 56;
const MAX_COMPANY_NAME_LENGTH: usize = 128;
const IDENTITY_INDEX_READY: &str = "identity_index_ready";

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Business {
    pub wallet: String,
    pub company_name: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrustSignal {
    pub signal_type: Symbol,
    pub value: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalRecord {
    pub business_id: u32,
    pub signal: TrustSignal,
}

/// A signal with an explicit positive integer weight.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeightedSignalRecord {
    pub business_id: u32,
    pub signal: TrustSignal,
    pub weight: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScoreRecord {
    pub business_id: u32,
    pub score: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BusinessProfile {
    pub business_id: u32,
    pub category: Symbol,
    pub tier: u32,
    pub active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VersionedBusinessProfile {
    pub version: u32,
    pub profile: BusinessProfile,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BusinessStats {
    pub business_id: u32,
    pub signal_count: u32,
    pub average_value: i128,
    pub has_signals: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TierSummary {
    pub tier: u32,
    pub business_count: u32,
    pub business_ids: Vec<u32>,
}

const PROFILE_STORAGE_VERSION: u32 = 1;

fn profile_storage_key(env: &Env) -> Symbol {
    Symbol::new(env, "profile")
}

fn read_versioned_profile(env: &Env, business_id: u32) -> Option<VersionedBusinessProfile> {
    let profiles: Map<u32, VersionedBusinessProfile> = env
        .storage()
        .persistent()
        .get(&profile_storage_key(env))
        .unwrap_or_else(|| Map::new(env));
    profiles.get(business_id)
}

fn read_legacy_profile(env: &Env, business_id: u32) -> BusinessProfile {
    let categories: Map<u32, Symbol> = env.storage().persistent()
        .get(&Symbol::new(env, "category")).unwrap_or_else(|| Map::new(env));
    let tiers: Map<u32, u32> = env.storage().persistent()
        .get(&Symbol::new(env, "tier")).unwrap_or_else(|| Map::new(env));
    let active: Map<u32, bool> = env.storage().persistent()
        .get(&Symbol::new(env, "active")).unwrap_or_else(|| Map::new(env));
    BusinessProfile {
        business_id,
        category: categories.get(business_id).unwrap_or_else(|| Symbol::new(env, "none")),
        tier: tiers.get(business_id).unwrap_or(0),
        active: active.get(business_id).unwrap_or(true),
    }
}

fn read_profile(env: &Env, business_id: u32) -> BusinessProfile {
    if let Some(record) = read_versioned_profile(env, business_id) {
        assert_eq!(record.version, PROFILE_STORAGE_VERSION, "unsupported profile storage version");
        record.profile
    } else {
        read_legacy_profile(env, business_id)
    }
}

fn write_profile(env: &Env, profile: BusinessProfile) {
    let mut profiles: Map<u32, VersionedBusinessProfile> = env.storage().persistent()
        .get(&profile_storage_key(env)).unwrap_or_else(|| Map::new(env));
    profiles.set(profile.business_id, VersionedBusinessProfile {
        version: PROFILE_STORAGE_VERSION,
        profile,
    });
    env.storage().persistent().set(&profile_storage_key(env), &profiles);
}

#[contract]
pub struct TrustLayerContract;

#[contractimpl]
impl TrustLayerContract {
    pub fn initialize(env: Env, authority: Address) {
        let key = Symbol::new(&env, AUTHORITY_KEY);
        if env.storage().persistent().has(&key) {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }
        authority.require_auth();
        env.storage().persistent().set(&key, &authority);
    }

    pub fn get_authority(env: Env) -> Option<Address> {
        env.storage().persistent().get(&Symbol::new(&env, AUTHORITY_KEY))
    }

    fn require_authority(env: &Env, caller: &Address) {
        let authority: Address = env
            .storage()
            .persistent()
            .get(&Symbol::new(env, AUTHORITY_KEY))
            .unwrap_or_else(|| panic_with_error!(env, Error::NotInitialized));
        if authority != *caller {
            panic_with_error!(env, Error::Unauthorized);
        }
        caller.require_auth();
    }

    fn supported_signal_types(env: &Env) -> Vec<Symbol> {
        let mut types = Vec::new(env);
        types.push_back(Symbol::new(env, "payment"));
        types.push_back(Symbol::new(env, "review"));
        types.push_back(Symbol::new(env, "delivery"));
        types.push_back(Symbol::new(env, "compliance"));
        types.push_back(Symbol::new(env, "dispute"));
        types
    }

    fn validate_signal(env: &Env, signal_type: &Symbol, value: i128) {
        let signal_text = SymbolStr::try_from_val(env, &signal_type.to_symbol_val()).unwrap();
        if signal_text.len() > SIGNAL_MAX_TYPE_LEN as usize {
            panic_with_error!(env, SignalError::SignalTypeTooLong);
        }
        let supported = Self::supported_signal_types(env);
        if signal_text.is_empty() || !supported.contains(signal_type) {
            panic_with_error!(env, SignalError::UnsupportedSignalType);
        }
        if !(SIGNAL_MIN_VALUE..=SIGNAL_MAX_VALUE).contains(&value) {
            panic_with_error!(env, SignalError::SignalValueOutOfBounds);
        }
    }

    /// Return the deterministic signal schema used by `record_signal`.
    pub fn get_signal_schema(env: Env) -> SignalSchema {
        SignalSchema {
            version: SIGNAL_SCHEMA_VERSION,
            min_value: SIGNAL_MIN_VALUE,
            max_value: SIGNAL_MAX_VALUE,
            max_type_len: SIGNAL_MAX_TYPE_LEN,
            allowed_types: Self::supported_signal_types(&env),
        }
    }

    /// Validate the canonical wallet representation used by new registrations.
    ///
    /// Stellar account identifiers are represented as an uppercase `G` followed
    /// by one to 55 uppercase alphanumeric characters.  The contract keeps the
    /// representation as a String for backwards compatibility with the
    /// original API, but validates its byte-level shape before persisting it.
    fn validate_wallet(wallet: &String) {
        let length = wallet.len() as usize;
        if !(2..=MAX_WALLET_LENGTH).contains(&length) {
            panic!("invalid business wallet length");
        }

        let mut bytes = [0u8; MAX_WALLET_LENGTH];
        wallet.copy_into_slice(&mut bytes[..length]);
        if bytes[0] != b'G' {
            panic!("business wallet must start with G");
        }
        for byte in &bytes[1..length] {
            if !((*byte >= b'A' && *byte <= b'Z') || (*byte >= b'0' && *byte <= b'9')) {
                panic!("business wallet contains a non-canonical character");
            }
        }
    }

    /// Validate the bounded business name without changing legacy records.
    fn validate_company_name(company_name: &String) {
        let length = company_name.len() as usize;
        if length == 0 || length > MAX_COMPANY_NAME_LENGTH {
            panic!("invalid company name length");
        }
    }

    fn identity_index_key(env: &Env) -> Symbol {
        Symbol::new(env, "identity_index")
    }

    fn identity_index_ready_key(env: &Env) -> Symbol {
        Symbol::new(env, IDENTITY_INDEX_READY)
    }

    /// Build the new wallet index once so records written by older versions
    /// remain readable.  Invalid legacy values are deliberately left alone;
    /// they can still be returned by `get_business`, but cannot collide with a
    /// new canonical registration.
    fn ensure_identity_index(env: &Env) -> Map<String, u32> {
        let ready_key = Self::identity_index_ready_key(env);
        if let Some(index) = env
            .storage()
            .persistent()
            .get(&Self::identity_index_key(env))
        {
            if env
                .storage()
                .persistent()
                .get::<_, bool>(&ready_key)
                .unwrap_or(false)
            {
                return index;
            }
        }

        let businesses_key = Symbol::new(env, "business");
        let businesses: Vec<Business> = env
            .storage()
            .persistent()
            .get(&businesses_key)
            .unwrap_or_else(|| Vec::new(env));
        let mut index: Map<String, u32> = Map::new(env);
        for id in 0..businesses.len() {
            let business = businesses.get(id).unwrap();
            if business.wallet.len() >= 2 && business.wallet.len() as usize <= MAX_WALLET_LENGTH {
                // Existing deployments may contain values that predate strict
                // validation.  Only index values that pass the same policy.
                let length = business.wallet.len() as usize;
                let mut bytes = [0u8; MAX_WALLET_LENGTH];
                business.wallet.copy_into_slice(&mut bytes[..length]);
                let valid = bytes[0] == b'G'
                    && bytes[1..length].iter().all(|byte| {
                        (*byte >= b'A' && *byte <= b'Z') || (*byte >= b'0' && *byte <= b'9')
                    });
                if valid && index.get(business.wallet.clone()).is_none() {
                    index.set(business.wallet, id);
                }
            }
        }
        env.storage()
            .persistent()
            .set(&Self::identity_index_key(env), &index);
        env.storage().persistent().set(&ready_key, &true);
        index
    }

    fn checked_add(left: i128, right: i128) -> i128 {
        left.checked_add(right)
            .unwrap_or_else(|| panic!("trust score arithmetic overflow"))
    }

    fn checked_mul(left: i128, right: i128) -> i128 {
        left.checked_mul(right)
            .unwrap_or_else(|| panic!("trust score arithmetic overflow"))
    }

    fn checked_div(numerator: i128, denominator: i128) -> i128 {
        numerator
            .checked_div(denominator)
            .unwrap_or_else(|| panic!("trust score division failed"))
    }

    /// Round non-negative division to the nearest integer, breaking ties up.
    fn rounded_average(total: i128, weight: i128) -> i128 {
        if weight == 0 {
            return 0;
        }
        let quotient = Self::checked_div(total, weight);
        let remainder = total % weight;
        let doubled_remainder = Self::checked_mul(remainder, 2);
        if doubled_remainder >= weight {
            quotient
                .checked_add(1)
                .unwrap_or_else(|| panic!("trust score arithmetic overflow"))
        } else {
            quotient
        }
    }

    /// Compute the one score used by update, verification, and statistics.
    /// Ordinary signals have weight one; weighted records use their explicit
    /// positive weight. Negative values are rejected at ingestion, and legacy
    /// negative records are ignored rather than allowing a negative score to
    /// bypass the non-negative policy.
    fn compute_score(env: &Env, business_id: u32) -> i128 {
        let signals_key = Symbol::new(env, "signals");
        let signals: Vec<SignalRecord> = env
            .storage()
            .persistent()
            .get(&signals_key)
            .unwrap_or_else(|| Vec::new(env));
        let weighted_key = Symbol::new(env, "weighted_signals");
        let weighted: Vec<WeightedSignalRecord> = env
            .storage()
            .persistent()
            .get(&weighted_key)
            .unwrap_or_else(|| Vec::new(env));
        let mut total = 0i128;
        let mut weight = 0i128;

        for i in 0..signals.len() {
            let record = signals.get(i).unwrap();
            if record.business_id == business_id && record.signal.value >= 0 {
                total = Self::checked_add(total, record.signal.value);
                weight = Self::checked_add(weight, 1);
            }
        }
        for i in 0..weighted.len() {
            let record = weighted.get(i).unwrap();
            if record.business_id == business_id && record.signal.value >= 0 {
                total =
                    Self::checked_add(total, Self::checked_mul(record.signal.value, record.weight));
                weight = Self::checked_add(weight, record.weight);
            }
        }
        Self::rounded_average(total, weight)
    }

    /// Register a business with wallet and company name.
    pub fn register_business(env: Env, caller: Address, wallet: String, company_name: String) -> u32 {
        Self::require_authority(&env, &caller);
        Self::validate_wallet(&wallet);
        Self::validate_company_name(&company_name);
        let mut identity_index = Self::ensure_identity_index(&env);
        if identity_index.get(wallet.clone()).is_some() {
            panic!("business wallet is already registered");
        }
        let business = Business {
            wallet: wallet.clone(),
            company_name: company_name.clone(),
        };
        let key = Symbol::new(&env, "business");
        let mut businesses: Vec<Business> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        let id = businesses.len();
        businesses.push_back(business);
        env.storage().persistent().set(&key, &businesses);
        identity_index.set(wallet, id);
        env.storage()
            .persistent()
            .set(&Self::identity_index_key(&env), &identity_index);
        id
    }

    /// Record a trust signal for a business.
    pub fn record_signal(
        env: Env,
        caller: Address,
        business_id: u32,
        signal_type: Symbol,
        value: i128,
    ) -> bool {
        Self::require_authority(&env, &caller);
        assert!(Self::is_active(env.clone(), business_id), "inactive business cannot receive trust data");
        Self::validate_signal(&env, &signal_type, value);
        if value < 0 {
            panic!("negative trust signals are not permitted");
        }
        let signal = TrustSignal {
            signal_type: signal_type.clone(),
            value,
        };
        let record = SignalRecord {
            business_id,
            signal,
        };
        let key = Symbol::new(&env, "signals");
        let mut signals: Vec<SignalRecord> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        signals.push_back(record);
        env.storage().persistent().set(&key, &signals);
        true
    }

    /// Record a non-negative signal with a checked positive integer weight.
    pub fn record_weighted_signal(
        env: Env,
        caller: Address,
        business_id: u32,
        signal_type: Symbol,
        value: i128,
        weight: i128,
    ) -> bool {
        Self::require_authority(&env, &caller);
        assert!(Self::is_active(env.clone(), business_id), "inactive business cannot receive trust data");
        if value < 0 {
            panic!("negative trust signals are not permitted");
        }
        if weight <= 0 {
            panic!("trust signal weight must be positive");
        }
        // Validate the multiplication before writing the record so an
        // overflowed contribution cannot leave a partially accepted signal.
        Self::checked_mul(value, weight);
        let record = WeightedSignalRecord {
            business_id,
            signal: TrustSignal { signal_type, value },
            weight,
        };
        let key = Symbol::new(&env, "weighted_signals");
        let mut records: Vec<WeightedSignalRecord> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        records.push_back(record);
        env.storage().persistent().set(&key, &records);
        true
    }

    /// Update trust score for a business (computed from signals).
    pub fn update_trust_score(env: Env, caller: Address, business_id: u32) -> i128 {
        Self::require_authority(&env, &caller);
        assert!(Self::is_active(env.clone(), business_id), "inactive business cannot receive trust data");
        let score = Self::compute_score(&env, business_id);
        let score_key = Symbol::new(&env, "score");
        let mut scores: Vec<ScoreRecord> = env
            .storage()
            .persistent()
            .get(&score_key)
            .unwrap_or_else(|| Vec::new(&env));
        let mut found = false;
        let score_len = scores.len();
        for i in 0..score_len {
            let rec = scores.get(i).unwrap();
            if rec.business_id == business_id {
                scores.set(i, ScoreRecord { business_id, score });
                found = true;
                break;
            }
        }
        if !found {
            scores.push_back(ScoreRecord { business_id, score });
        }
        env.storage().persistent().set(&score_key, &scores);
        score
    }

    /// Verify and return trust score for a business.
    pub fn verify_trust_score(env: Env, business_id: u32) -> i128 {
        Self::compute_score(&env, business_id)
    }

    /// Set the business category for a business profile.
    pub fn set_category(env: Env, caller: Address, business_id: u32, category: Symbol) {
        Self::require_authority(&env, &caller);
        let mut profile = read_profile(&env, business_id);
        profile.category = category;
        write_profile(&env, profile);
    }

    /// Get the business category, defaulting to "none" when unset.
    pub fn get_category(env: Env, business_id: u32) -> Symbol {
        read_profile(&env, business_id).category
    }

    /// Set the verification tier for a business.
    pub fn set_verification_tier(env: Env, caller: Address, business_id: u32, tier: u32) {
        Self::require_authority(&env, &caller);
        let mut profile = read_profile(&env, business_id);
        profile.tier = tier;
        write_profile(&env, profile);
    }

    /// Get the verification tier for a business, defaulting to 0.
    pub fn get_verification_tier(env: Env, business_id: u32) -> u32 {
        read_profile(&env, business_id).tier
    }

    /// Deactivate a business, marking it inactive in the profile store.
    pub fn deactivate_business(env: Env, caller: Address, business_id: u32) {
        Self::require_authority(&env, &caller);
        let mut profile = read_profile(&env, business_id);
        profile.active = false;
        write_profile(&env, profile);
    }

    /// Reactivate a business, marking it active in the profile store.
    pub fn reactivate_business(env: Env, caller: Address, business_id: u32) {
        Self::require_authority(&env, &caller);
        let mut profile = read_profile(&env, business_id);
        profile.active = true;
        write_profile(&env, profile);
    }

    /// Report whether a business is active, defaulting to true.
    pub fn is_active(env: Env, business_id: u32) -> bool {
        read_profile(&env, business_id).active
    }

    /// Get a registered business by id, or None when out of range.
    pub fn get_business(env: Env, business_id: u32) -> Option<Business> {
        let key = Symbol::new(&env, "business");
        let businesses: Vec<Business> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        if business_id < businesses.len() {
            Some(businesses.get(business_id).unwrap())
        } else {
            None
        }
    }

    /// Return the first business registered with a canonical wallet.
    pub fn get_business_by_wallet(env: Env, wallet: String) -> Option<u32> {
        Self::validate_wallet(&wallet);
        Self::ensure_identity_index(&env).get(wallet)
    }

    /// Report whether a canonical wallet is already present in the registry.
    pub fn is_wallet_registered(env: Env, wallet: String) -> bool {
        Self::get_business_by_wallet(env, wallet).is_some()
    }

    /// Count the number of registered businesses.
    pub fn count_businesses(env: Env) -> u32 {
        let key = Symbol::new(&env, "business");
        let businesses: Vec<Business> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        businesses.len()
    }

    /// Report whether a business meets a required verification tier.
    pub fn meets_tier(env: Env, business_id: u32, required: u32) -> bool {
        Self::get_verification_tier(env, business_id) >= required
    }

    /// Register a business and immediately set its verification tier.
    pub fn register_verified_business(
        env: Env,
        caller: Address,
        wallet: String,
        company_name: String,
        tier: u32,
    ) -> u32 {
        Self::require_authority(&env, &caller);
        let id = Self::register_business(env.clone(), caller.clone(), wallet, company_name);
        Self::set_verification_tier(env, caller, id, tier);
        id
    }

    /// Aggregate category, tier, and active status into a profile view.
    pub fn get_profile(env: Env, business_id: u32) -> BusinessProfile {
        read_profile(&env, business_id)
    }

    /// Report whether a business has a verification tier of at least one.
    pub fn is_verified(env: Env, business_id: u32) -> bool {
        Self::get_verification_tier(env, business_id) >= 1
    }

    /// Increment a business's verification tier by one and return the new tier.
    pub fn bump_tier(env: Env, caller: Address, business_id: u32) -> u32 {
        Self::require_authority(&env, &caller);
        let next = Self::get_verification_tier(env.clone(), business_id) + 1;
        Self::set_verification_tier(env, caller, business_id, next);
        next
    }

    /// Decrease a business's verification tier by one, never below zero.
    pub fn downgrade_tier(env: Env, caller: Address, business_id: u32) -> u32 {
        Self::require_authority(&env, &caller);
        let current = Self::get_verification_tier(env.clone(), business_id);
        let next = if current > 0 { current - 1 } else { 0 };
        Self::set_verification_tier(env, caller, business_id, next);
        next
    }

    /// Set category, tier, and active status for a business in a single call.
    pub fn set_profile(
        env: Env, caller: Address, business_id: u32, category: Symbol, tier: u32, active: bool,
    ) {
        Self::require_authority(&env, &caller);
        write_profile(&env, BusinessProfile { business_id, category, tier, active });
        env.events().publish((Symbol::new(&env, "profile_changed"),), business_id);
    }

    /// Count registered businesses that are currently active.
    pub fn count_active_businesses(env: Env) -> u32 {
        let total = Self::count_businesses(env.clone());
        let mut count: u32 = 0;
        for id in 0..total {
            if Self::is_active(env.clone(), id) {
                count += 1;
            }
        }
        count
    }

    /// Report whether a business is both active and verified.
    pub fn is_active_and_verified(env: Env, business_id: u32) -> bool {
        Self::is_active(env.clone(), business_id) && Self::is_verified(env, business_id)
    }

    /// Count how many signals have been recorded for a business.
    pub fn count_signals_for_business(env: Env, business_id: u32) -> u32 {
        let key = Symbol::new(&env, "signals");
        let signals: Vec<SignalRecord> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        let mut count: u32 = 0;
        let len = signals.len();
        for i in 0..len {
            let record = signals.get(i).unwrap();
            if record.business_id == business_id {
                count += 1;
            }
        }
        count
    }

    /// Report whether a business has at least one recorded signal.
    pub fn has_signals(env: Env, business_id: u32) -> bool {
        Self::count_signals_for_business(env, business_id) > 0
    }

    /// Return the value of the most recently recorded signal for a business.
    pub fn latest_signal_value(env: Env, business_id: u32) -> Option<i128> {
        let key = Symbol::new(&env, "signals");
        let signals: Vec<SignalRecord> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        let mut latest: Option<i128> = None;
        let len = signals.len();
        for i in 0..len {
            let record = signals.get(i).unwrap();
            if record.business_id == business_id {
                latest = Some(record.signal.value);
            }
        }
        latest
    }

    /// Average raw signal value for a business; zero when it has none.
    pub fn average_signal_value(env: Env, business_id: u32) -> i128 {
        Self::compute_score(&env, business_id)
    }

    /// Count signals of a specific type recorded for a business.
    pub fn signal_type_count(env: Env, business_id: u32, signal_type: Symbol) -> u32 {
        let key = Symbol::new(&env, "signals");
        let signals: Vec<SignalRecord> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        let mut count: u32 = 0;
        let len = signals.len();
        for i in 0..len {
            let record = signals.get(i).unwrap();
            if record.business_id == business_id && record.signal.signal_type == signal_type {
                count += 1;
            }
        }
        count
    }

    /// Aggregate signal count, average value, and presence into a stats view.
    pub fn get_business_stats(env: Env, business_id: u32) -> BusinessStats {
        let signal_count = Self::count_signals_for_business(env.clone(), business_id);
        let average_value = Self::average_signal_value(env.clone(), business_id);
        let has_signals = Self::has_signals(env, business_id);
        BusinessStats {
            business_id,
            signal_count,
            average_value,
            has_signals,
        }
    }

    /// Count registered businesses whose verification tier equals `tier`.
    pub fn count_businesses_at_tier(env: Env, tier: u32) -> u32 {
        let total = Self::count_businesses(env.clone());
        let mut count: u32 = 0;
        for id in 0..total {
            if Self::is_active(env.clone(), id) && Self::get_verification_tier(env.clone(), id) == tier {
                count += 1;
            }
        }
        count
    }

    /// List the ids of registered businesses whose verification tier equals `tier`.
    pub fn list_business_ids_at_tier(env: Env, tier: u32) -> Vec<u32> {
        let total = Self::count_businesses(env.clone());
        let mut ids: Vec<u32> = Vec::new(&env);
        for id in 0..total {
            if Self::is_active(env.clone(), id) && Self::get_verification_tier(env.clone(), id) == tier {
                ids.push_back(id);
            }
        }
        ids
    }

    /// Highest verification tier among registered businesses; zero when none exist.
    pub fn highest_tier(env: Env) -> u32 {
        let total = Self::count_businesses(env.clone());
        let mut highest: u32 = 0;
        for id in 0..total {
            if Self::is_active(env.clone(), id) {
                let tier = Self::get_verification_tier(env.clone(), id);
                if tier > highest {
                    highest = tier;
                }
            }
        }
        highest
    }

    /// List the ids of registered businesses meeting a required verification tier.
    pub fn list_business_ids_meeting_tier(env: Env, required: u32) -> Vec<u32> {
        let total = Self::count_businesses(env.clone());
        let mut ids: Vec<u32> = Vec::new(&env);
        for id in 0..total {
            if Self::is_active(env.clone(), id) && Self::meets_tier(env.clone(), id, required) {
                ids.push_back(id);
            }
        }
        ids
    }

    /// Count registered businesses assigned to a given category.
    pub fn count_businesses_in_category(env: Env, category: Symbol) -> u32 {
        let total = Self::count_businesses(env.clone());
        let mut count: u32 = 0;
        for id in 0..total {
            if Self::is_active(env.clone(), id) && Self::get_category(env.clone(), id) == category {
                count += 1;
            }
        }
        count
    }

    /// List the ids of registered businesses assigned to a given category.
    pub fn list_business_ids_in_category(env: Env, category: Symbol) -> Vec<u32> {
        let total = Self::count_businesses(env.clone());
        let mut ids: Vec<u32> = Vec::new(&env);
        for id in 0..total {
            if Self::is_active(env.clone(), id) && Self::get_category(env.clone(), id) == category {
                ids.push_back(id);
            }
        }
        ids
    }

    /// Aggregate business count and ids for a given verification tier.
    pub fn get_tier_summary(env: Env, tier: u32) -> TierSummary {
        let business_ids = Self::list_business_ids_at_tier(env.clone(), tier);
        let business_count = business_ids.len();
        TierSummary {
            tier,
            business_count,
            business_ids,
        }
    }
}

mod test;
