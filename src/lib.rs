#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Map, String, Symbol, Vec};

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

#[contract]
pub struct TrustLayerContract;

#[contractimpl]
impl TrustLayerContract {
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
    pub fn register_business(env: Env, wallet: String, company_name: String) -> u32 {
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
        id
    }

    /// Record a trust signal for a business.
    pub fn record_signal(env: Env, business_id: u32, signal_type: Symbol, value: i128) -> bool {
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
        business_id: u32,
        signal_type: Symbol,
        value: i128,
        weight: i128,
    ) -> bool {
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
    pub fn update_trust_score(env: Env, business_id: u32) -> i128 {
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
    pub fn set_category(env: Env, business_id: u32, category: Symbol) {
        let key = Symbol::new(&env, "category");
        let mut categories: Map<u32, Symbol> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Map::new(&env));
        categories.set(business_id, category);
        env.storage().persistent().set(&key, &categories);
    }

    /// Get the business category, defaulting to "none" when unset.
    pub fn get_category(env: Env, business_id: u32) -> Symbol {
        let key = Symbol::new(&env, "category");
        let categories: Map<u32, Symbol> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Map::new(&env));
        categories
            .get(business_id)
            .unwrap_or_else(|| Symbol::new(&env, "none"))
    }

    /// Set the verification tier for a business.
    pub fn set_verification_tier(env: Env, business_id: u32, tier: u32) {
        let key = Symbol::new(&env, "tier");
        let mut tiers: Map<u32, u32> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Map::new(&env));
        tiers.set(business_id, tier);
        env.storage().persistent().set(&key, &tiers);
    }

    /// Get the verification tier for a business, defaulting to 0.
    pub fn get_verification_tier(env: Env, business_id: u32) -> u32 {
        let key = Symbol::new(&env, "tier");
        let tiers: Map<u32, u32> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Map::new(&env));
        tiers.get(business_id).unwrap_or(0)
    }

    /// Deactivate a business, marking it inactive in the profile store.
    pub fn deactivate_business(env: Env, business_id: u32) {
        let key = Symbol::new(&env, "active");
        let mut active: Map<u32, bool> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Map::new(&env));
        active.set(business_id, false);
        env.storage().persistent().set(&key, &active);
    }

    /// Reactivate a business, marking it active in the profile store.
    pub fn reactivate_business(env: Env, business_id: u32) {
        let key = Symbol::new(&env, "active");
        let mut active: Map<u32, bool> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Map::new(&env));
        active.set(business_id, true);
        env.storage().persistent().set(&key, &active);
    }

    /// Report whether a business is active, defaulting to true.
    pub fn is_active(env: Env, business_id: u32) -> bool {
        let key = Symbol::new(&env, "active");
        let active: Map<u32, bool> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Map::new(&env));
        active.get(business_id).unwrap_or(true)
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
        wallet: String,
        company_name: String,
        tier: u32,
    ) -> u32 {
        let id = Self::register_business(env.clone(), wallet, company_name);
        Self::set_verification_tier(env, id, tier);
        id
    }

    /// Aggregate category, tier, and active status into a profile view.
    pub fn get_profile(env: Env, business_id: u32) -> BusinessProfile {
        let category = Self::get_category(env.clone(), business_id);
        let tier = Self::get_verification_tier(env.clone(), business_id);
        let active = Self::is_active(env, business_id);
        BusinessProfile {
            business_id,
            category,
            tier,
            active,
        }
    }

    /// Report whether a business has a verification tier of at least one.
    pub fn is_verified(env: Env, business_id: u32) -> bool {
        Self::get_verification_tier(env, business_id) >= 1
    }

    /// Increment a business's verification tier by one and return the new tier.
    pub fn bump_tier(env: Env, business_id: u32) -> u32 {
        let next = Self::get_verification_tier(env.clone(), business_id) + 1;
        Self::set_verification_tier(env, business_id, next);
        next
    }

    /// Decrease a business's verification tier by one, never below zero.
    pub fn downgrade_tier(env: Env, business_id: u32) -> u32 {
        let current = Self::get_verification_tier(env.clone(), business_id);
        let next = if current > 0 { current - 1 } else { 0 };
        Self::set_verification_tier(env, business_id, next);
        next
    }

    /// Set category, tier, and active status for a business in a single call.
    pub fn set_profile(env: Env, business_id: u32, category: Symbol, tier: u32, active: bool) {
        Self::set_category(env.clone(), business_id, category);
        Self::set_verification_tier(env.clone(), business_id, tier);
        if active {
            Self::reactivate_business(env, business_id);
        } else {
            Self::deactivate_business(env, business_id);
        }
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
            if Self::get_verification_tier(env.clone(), id) == tier {
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
            if Self::get_verification_tier(env.clone(), id) == tier {
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
            let tier = Self::get_verification_tier(env.clone(), id);
            if tier > highest {
                highest = tier;
            }
        }
        highest
    }

    /// List the ids of registered businesses meeting a required verification tier.
    pub fn list_business_ids_meeting_tier(env: Env, required: u32) -> Vec<u32> {
        let total = Self::count_businesses(env.clone());
        let mut ids: Vec<u32> = Vec::new(&env);
        for id in 0..total {
            if Self::meets_tier(env.clone(), id, required) {
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
            if Self::get_category(env.clone(), id) == category {
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
            if Self::get_category(env.clone(), id) == category {
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
