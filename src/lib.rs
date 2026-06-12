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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScoreRecord {
    pub business_id: u32,
    pub score: i128,
}

#[contract]
pub struct TrustLayerContract;

#[contractimpl]
impl TrustLayerContract {
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

    /// Update trust score for a business (computed from signals).
    pub fn update_trust_score(env: Env, business_id: u32) -> i128 {
        let key = Symbol::new(&env, "signals");
        let signals: Vec<SignalRecord> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        let mut total: i128 = 0;
        let mut count: i128 = 0;
        let len = signals.len();
        for i in 0..len {
            let record = signals.get(i).unwrap();
            if record.business_id == business_id {
                total += record.signal.value;
                count += 1;
            }
        }
        let score = if count > 0 { total / count } else { 0 };
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
        let score_key = Symbol::new(&env, "score");
        let scores: Vec<ScoreRecord> = env
            .storage()
            .persistent()
            .get(&score_key)
            .unwrap_or_else(|| Vec::new(&env));
        let len = scores.len();
        for i in 0..len {
            let rec = scores.get(i).unwrap();
            if rec.business_id == business_id {
                return rec.score;
            }
        }
        0
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
}

mod test;
