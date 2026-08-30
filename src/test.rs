#![cfg(test)]

extern crate std;

use super::{
    Business, BusinessProfile, BusinessStats, TierSummary, TrustLayerContract,
    TrustLayerContractClient as GeneratedClient,
};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env, String, Symbol};

/// Test facade that preserves the concise pre-authorization test API while
/// making every state-changing call explicit about its authorized caller.
///
/// The production client intentionally exposes the caller as an argument. The
/// facade centralizes setup for the legacy behavioral tests; dedicated tests
/// below use the generated client directly to exercise unauthorized callers.
struct TrustLayerContractClient<'a> {
    inner: GeneratedClient<'a>,
    authority: Address,
}

impl<'a> TrustLayerContractClient<'a> {
    fn new(env: &'a Env, contract_id: &Address) -> Self {
        env.mock_all_auths();
        let authority = Address::generate(env);
        let inner = GeneratedClient::new(env, contract_id);
        inner.initialize(&authority);
        Self { inner, authority }
    }

    fn register_business(&self, wallet: &String, company_name: &String) -> u32 {
        self.inner
            .register_business(&self.authority, wallet, company_name)
    }

    fn record_signal(&self, business_id: &u32, signal_type: &Symbol, value: &i128) -> bool {
        self.inner
            .record_signal(&self.authority, business_id, signal_type, value)
    }

    fn update_trust_score(&self, business_id: &u32) -> i128 {
        self.inner.update_trust_score(&self.authority, business_id)
    }

    fn verify_trust_score(&self, business_id: &u32) -> i128 {
        self.inner.verify_trust_score(business_id)
    }

    fn set_category(&self, business_id: &u32, category: &Symbol) {
        self.inner
            .set_category(&self.authority, business_id, category);
    }

    fn get_category(&self, business_id: &u32) -> Symbol {
        self.inner.get_category(business_id)
    }

    fn set_verification_tier(&self, business_id: &u32, tier: &u32) {
        self.inner
            .set_verification_tier(&self.authority, business_id, tier);
    }

    fn get_verification_tier(&self, business_id: &u32) -> u32 {
        self.inner.get_verification_tier(business_id)
    }

    fn deactivate_business(&self, business_id: &u32) {
        self.inner.deactivate_business(&self.authority, business_id);
    }

    fn reactivate_business(&self, business_id: &u32) {
        self.inner.reactivate_business(&self.authority, business_id);
    }

    fn is_active(&self, business_id: &u32) -> bool {
        self.inner.is_active(business_id)
    }

    fn get_business(&self, business_id: &u32) -> Option<Business> {
        self.inner.get_business(business_id)
    }

    fn count_businesses(&self) -> u32 {
        self.inner.count_businesses()
    }

    fn meets_tier(&self, business_id: &u32, required: &u32) -> bool {
        self.inner.meets_tier(business_id, required)
    }

    fn register_verified_business(
        &self,
        wallet: &String,
        company_name: &String,
        tier: &u32,
    ) -> u32 {
        self.inner
            .register_verified_business(&self.authority, wallet, company_name, tier)
    }

    fn get_profile(&self, business_id: &u32) -> BusinessProfile {
        self.inner.get_profile(business_id)
    }

    fn is_verified(&self, business_id: &u32) -> bool {
        self.inner.is_verified(business_id)
    }

    fn bump_tier(&self, business_id: &u32) -> u32 {
        self.inner.bump_tier(&self.authority, business_id)
    }

    fn downgrade_tier(&self, business_id: &u32) -> u32 {
        self.inner.downgrade_tier(&self.authority, business_id)
    }

    fn set_profile(&self, business_id: &u32, category: &Symbol, tier: &u32, active: &bool) {
        self.inner
            .set_profile(&self.authority, business_id, category, tier, active);
    }

    fn count_active_businesses(&self) -> u32 {
        self.inner.count_active_businesses()
    }

    fn is_active_and_verified(&self, business_id: &u32) -> bool {
        self.inner.is_active_and_verified(business_id)
    }

    fn count_signals_for_business(&self, business_id: &u32) -> u32 {
        self.inner.count_signals_for_business(business_id)
    }

    fn has_signals(&self, business_id: &u32) -> bool {
        self.inner.has_signals(business_id)
    }

    fn latest_signal_value(&self, business_id: &u32) -> Option<i128> {
        self.inner.latest_signal_value(business_id)
    }

    fn average_signal_value(&self, business_id: &u32) -> i128 {
        self.inner.average_signal_value(business_id)
    }

    fn signal_type_count(&self, business_id: &u32, signal_type: &Symbol) -> u32 {
        self.inner.signal_type_count(business_id, signal_type)
    }

    fn get_business_stats(&self, business_id: &u32) -> BusinessStats {
        self.inner.get_business_stats(business_id)
    }

    fn count_businesses_at_tier(&self, tier: &u32) -> u32 {
        self.inner.count_businesses_at_tier(tier)
    }

    fn list_business_ids_at_tier(&self, tier: &u32) -> soroban_sdk::Vec<u32> {
        self.inner.list_business_ids_at_tier(tier)
    }

    fn highest_tier(&self) -> u32 {
        self.inner.highest_tier()
    }

    fn list_business_ids_meeting_tier(&self, required: &u32) -> soroban_sdk::Vec<u32> {
        self.inner.list_business_ids_meeting_tier(required)
    }

    fn count_businesses_in_category(&self, category: &Symbol) -> u32 {
        self.inner.count_businesses_in_category(category)
    }

    fn list_business_ids_in_category(&self, category: &Symbol) -> soroban_sdk::Vec<u32> {
        self.inner.list_business_ids_in_category(category)
    }

    fn get_tier_summary(&self, tier: &u32) -> TierSummary {
        self.inner.get_tier_summary(tier)
    }
}

#[test]
fn test_register_business() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    let id = client.register_business(
        &String::from_str(&env, "GABC..."),
        &String::from_str(&env, "Alpha Logistics"),
    );
    assert_eq!(id, 0);

    let id2 = client.register_business(
        &String::from_str(&env, "GDEF..."),
        &String::from_str(&env, "Beta Corp"),
    );
    assert_eq!(id2, 1);
}

#[test]
fn test_record_signal_and_verify_trust_score() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    let _ = client.register_business(
        &String::from_str(&env, "GABC..."),
        &String::from_str(&env, "Alpha Logistics"),
    );

    client.record_signal(&0, &Symbol::new(&env, "payment"), &100);
    client.record_signal(&0, &Symbol::new(&env, "payment"), &200);
    let score = client.update_trust_score(&0);
    assert_eq!(score, 150);

    let verified = client.verify_trust_score(&0);
    assert_eq!(verified, 150);
}

#[test]
fn test_verify_trust_score_unknown_business() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    let score = client.verify_trust_score(&99);
    assert_eq!(score, 0);
}

#[test]
fn test_set_and_get_category() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_category(&0, &Symbol::new(&env, "logistics"));
    assert_eq!(client.get_category(&0), Symbol::new(&env, "logistics"));
}

#[test]
fn test_get_category_default_none() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.get_category(&7), Symbol::new(&env, "none"));
}

#[test]
fn test_set_and_get_verification_tier() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_verification_tier(&0, &3);
    assert_eq!(client.get_verification_tier(&0), 3);
}

#[test]
fn test_get_verification_tier_default_zero() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.get_verification_tier(&5), 0);
}

#[test]
fn test_deactivate_business_sets_inactive() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.deactivate_business(&0);
    assert_eq!(client.is_active(&0), false);
}

#[test]
fn test_reactivate_business_sets_active() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.deactivate_business(&0);
    client.reactivate_business(&0);
    assert_eq!(client.is_active(&0), true);
}

#[test]
fn test_is_active_default_true() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.is_active(&42), true);
}

#[test]
fn test_get_business_existing() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    let id = client.register_business(
        &String::from_str(&env, "GABC..."),
        &String::from_str(&env, "Alpha Logistics"),
    );
    let business = client.get_business(&id).unwrap();
    assert_eq!(
        business.company_name,
        String::from_str(&env, "Alpha Logistics")
    );
}

#[test]
fn test_get_business_out_of_range_none() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.get_business(&3), None);
}

#[test]
fn test_count_businesses() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.count_businesses(), 0);
    client.register_business(
        &String::from_str(&env, "GABC..."),
        &String::from_str(&env, "Alpha Logistics"),
    );
    client.register_business(
        &String::from_str(&env, "GDEF..."),
        &String::from_str(&env, "Beta Corp"),
    );
    assert_eq!(client.count_businesses(), 2);
}

#[test]
fn test_meets_tier_true() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_verification_tier(&0, &3);
    assert_eq!(client.meets_tier(&0, &2), true);
}

#[test]
fn test_meets_tier_false() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_verification_tier(&0, &1);
    assert_eq!(client.meets_tier(&0, &3), false);
}

#[test]
fn test_meets_tier_equal_boundary() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_verification_tier(&0, &2);
    assert_eq!(client.meets_tier(&0, &2), true);
}

#[test]
fn test_meets_tier_default_zero_fails_requirement() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.meets_tier(&0, &1), false);
}

#[test]
fn test_register_verified_business_sets_tier() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    let id = client.register_verified_business(
        &String::from_str(&env, "GABC..."),
        &String::from_str(&env, "Alpha Logistics"),
        &4,
    );
    assert_eq!(id, 0);
    assert_eq!(client.get_verification_tier(&id), 4);
}

#[test]
fn test_register_verified_business_also_registers() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    let id = client.register_verified_business(
        &String::from_str(&env, "GABC..."),
        &String::from_str(&env, "Alpha Logistics"),
        &2,
    );
    assert_eq!(client.count_businesses(), 1);
    let business = client.get_business(&id).unwrap();
    assert_eq!(business.wallet, String::from_str(&env, "GABC..."));
}

#[test]
fn test_get_profile_aggregates_all_fields() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_category(&0, &Symbol::new(&env, "logistics"));
    client.set_verification_tier(&0, &3);
    client.deactivate_business(&0);

    let profile = client.get_profile(&0);
    assert_eq!(profile.business_id, 0);
    assert_eq!(profile.category, Symbol::new(&env, "logistics"));
    assert_eq!(profile.tier, 3);
    assert_eq!(profile.active, false);
}

#[test]
fn test_get_profile_defaults() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    let profile = client.get_profile(&9);
    assert_eq!(profile.business_id, 9);
    assert_eq!(profile.category, Symbol::new(&env, "none"));
    assert_eq!(profile.tier, 0);
    assert_eq!(profile.active, true);
}

#[test]
fn test_set_category_overwrite() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_category(&0, &Symbol::new(&env, "retail"));
    client.set_category(&0, &Symbol::new(&env, "finance"));
    assert_eq!(client.get_category(&0), Symbol::new(&env, "finance"));
}

#[test]
fn test_set_verification_tier_overwrite() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_verification_tier(&0, &1);
    client.set_verification_tier(&0, &5);
    assert_eq!(client.get_verification_tier(&0), 5);
}

#[test]
fn test_category_isolation_between_businesses() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_category(&0, &Symbol::new(&env, "retail"));
    client.set_category(&1, &Symbol::new(&env, "logistics"));
    assert_eq!(client.get_category(&0), Symbol::new(&env, "retail"));
    assert_eq!(client.get_category(&1), Symbol::new(&env, "logistics"));
}

#[test]
fn test_tier_isolation_between_businesses() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_verification_tier(&0, &2);
    client.set_verification_tier(&1, &4);
    assert_eq!(client.get_verification_tier(&0), 2);
    assert_eq!(client.get_verification_tier(&1), 4);
}

#[test]
fn test_active_isolation_between_businesses() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.deactivate_business(&0);
    assert_eq!(client.is_active(&0), false);
    assert_eq!(client.is_active(&1), true);
}

#[test]
fn test_profile_survives_reactivation() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_category(&0, &Symbol::new(&env, "retail"));
    client.set_verification_tier(&0, &2);
    client.deactivate_business(&0);
    client.reactivate_business(&0);

    let profile = client.get_profile(&0);
    assert_eq!(profile.category, Symbol::new(&env, "retail"));
    assert_eq!(profile.tier, 2);
    assert_eq!(profile.active, true);
}

#[test]
fn test_count_businesses_empty() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.count_businesses(), 0);
}

#[test]
fn test_is_verified_reflects_tier() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.is_verified(&0), false);
    client.set_verification_tier(&0, &1);
    assert_eq!(client.is_verified(&0), true);
}

#[test]
fn test_is_verified_default_false() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.is_verified(&7), false);
}

#[test]
fn test_bump_tier_increments_and_returns() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.bump_tier(&0), 1);
    assert_eq!(client.bump_tier(&0), 2);
    assert_eq!(client.get_verification_tier(&0), 2);
}

#[test]
fn test_downgrade_tier_floors_at_zero() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_verification_tier(&0, &2);
    assert_eq!(client.downgrade_tier(&0), 1);
    assert_eq!(client.downgrade_tier(&0), 0);
    assert_eq!(client.downgrade_tier(&0), 0);
}

#[test]
fn test_bump_then_downgrade_round_trip() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.bump_tier(&0);
    client.bump_tier(&0);
    client.downgrade_tier(&0);
    assert_eq!(client.get_verification_tier(&0), 1);
}

#[test]
fn test_set_profile_sets_all_fields() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_profile(&0, &Symbol::new(&env, "retail"), &3, &false);
    let profile = client.get_profile(&0);
    assert_eq!(profile.category, Symbol::new(&env, "retail"));
    assert_eq!(profile.tier, 3);
    assert_eq!(profile.active, false);
}

#[test]
fn test_set_profile_active_true() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_profile(&0, &Symbol::new(&env, "fintech"), &5, &true);
    assert_eq!(client.is_active(&0), true);
    assert_eq!(client.is_active_and_verified(&0), true);
}

#[test]
fn test_is_active_and_verified_false_when_inactive() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.set_verification_tier(&0, &2);
    client.deactivate_business(&0);
    assert_eq!(client.is_active_and_verified(&0), false);
}

#[test]
fn test_count_active_businesses() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G1"),
        &String::from_str(&env, "One"),
    );
    client.register_business(
        &String::from_str(&env, "G2"),
        &String::from_str(&env, "Two"),
    );
    client.register_business(
        &String::from_str(&env, "G3"),
        &String::from_str(&env, "Three"),
    );
    assert_eq!(client.count_active_businesses(), 3);
    client.deactivate_business(&1);
    assert_eq!(client.count_active_businesses(), 2);
}

#[test]
fn test_count_signals_for_business_with_no_signals() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.count_signals_for_business(&0), 0);
}

#[test]
fn test_count_signals_for_business_counts_only_matching_business() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.record_signal(&0, &Symbol::new(&env, "payment"), &100);
    client.record_signal(&0, &Symbol::new(&env, "review"), &50);
    client.record_signal(&1, &Symbol::new(&env, "payment"), &75);

    assert_eq!(client.count_signals_for_business(&0), 2);
    assert_eq!(client.count_signals_for_business(&1), 1);
}

#[test]
fn test_has_signals_false_by_default() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.has_signals(&0), false);
}

#[test]
fn test_has_signals_true_after_recording_a_signal() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.record_signal(&0, &Symbol::new(&env, "payment"), &100);
    assert_eq!(client.has_signals(&0), true);
}

#[test]
fn test_latest_signal_value_none_when_empty() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.latest_signal_value(&0), None);
}

#[test]
fn test_latest_signal_value_returns_the_most_recent_value() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.record_signal(&0, &Symbol::new(&env, "payment"), &100);
    client.record_signal(&0, &Symbol::new(&env, "review"), &42);

    assert_eq!(client.latest_signal_value(&0), Some(42));
}

#[test]
fn test_average_signal_value_zero_when_empty() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.average_signal_value(&0), 0);
}

#[test]
fn test_average_signal_value_averages_recorded_values() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.record_signal(&0, &Symbol::new(&env, "payment"), &100);
    client.record_signal(&0, &Symbol::new(&env, "payment"), &200);

    assert_eq!(client.average_signal_value(&0), 150);
}

#[test]
fn test_signal_type_count_filters_by_type() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.record_signal(&0, &Symbol::new(&env, "payment"), &100);
    client.record_signal(&0, &Symbol::new(&env, "payment"), &50);
    client.record_signal(&0, &Symbol::new(&env, "review"), &10);
    client.record_signal(&1, &Symbol::new(&env, "payment"), &75);

    assert_eq!(
        client.signal_type_count(&0, &Symbol::new(&env, "payment")),
        2
    );
    assert_eq!(
        client.signal_type_count(&0, &Symbol::new(&env, "review")),
        1
    );
    assert_eq!(
        client.signal_type_count(&0, &Symbol::new(&env, "dispute")),
        0
    );
}

#[test]
fn test_get_business_stats_aggregates_all_fields() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.record_signal(&0, &Symbol::new(&env, "payment"), &100);
    client.record_signal(&0, &Symbol::new(&env, "payment"), &200);

    let stats = client.get_business_stats(&0);
    assert_eq!(stats.business_id, 0);
    assert_eq!(stats.signal_count, 2);
    assert_eq!(stats.average_value, 150);
    assert_eq!(stats.has_signals, true);
}

#[test]
fn test_count_businesses_at_tier_zero_when_none_registered() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.count_businesses_at_tier(&2), 0);
}

#[test]
fn test_count_businesses_at_tier_counts_only_matching_tier() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G1"),
        &String::from_str(&env, "One"),
    );
    client.register_business(
        &String::from_str(&env, "G2"),
        &String::from_str(&env, "Two"),
    );
    client.register_business(
        &String::from_str(&env, "G3"),
        &String::from_str(&env, "Three"),
    );
    client.set_verification_tier(&0, &2);
    client.set_verification_tier(&1, &2);
    client.set_verification_tier(&2, &1);

    assert_eq!(client.count_businesses_at_tier(&2), 2);
    assert_eq!(client.count_businesses_at_tier(&1), 1);
}

#[test]
fn test_list_business_ids_at_tier_returns_matching_ids() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G1"),
        &String::from_str(&env, "One"),
    );
    client.register_business(
        &String::from_str(&env, "G2"),
        &String::from_str(&env, "Two"),
    );
    client.set_verification_tier(&0, &3);
    client.set_verification_tier(&1, &1);

    let ids = client.list_business_ids_at_tier(&3);
    assert_eq!(ids.len(), 1);
    assert_eq!(ids.get(0), Some(0));
}

#[test]
fn test_highest_tier_zero_when_no_businesses() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_eq!(client.highest_tier(), 0);
}

#[test]
fn test_highest_tier_returns_the_max_tier() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G1"),
        &String::from_str(&env, "One"),
    );
    client.register_business(
        &String::from_str(&env, "G2"),
        &String::from_str(&env, "Two"),
    );
    client.set_verification_tier(&0, &2);
    client.set_verification_tier(&1, &4);

    assert_eq!(client.highest_tier(), 4);
}

#[test]
fn test_list_business_ids_meeting_tier_filters_by_threshold() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G1"),
        &String::from_str(&env, "One"),
    );
    client.register_business(
        &String::from_str(&env, "G2"),
        &String::from_str(&env, "Two"),
    );
    client.register_business(
        &String::from_str(&env, "G3"),
        &String::from_str(&env, "Three"),
    );
    client.set_verification_tier(&0, &3);
    client.set_verification_tier(&1, &1);
    client.set_verification_tier(&2, &2);

    let ids = client.list_business_ids_meeting_tier(&2);
    assert_eq!(ids.len(), 2);
    assert_eq!(ids.get(0), Some(0));
    assert_eq!(ids.get(1), Some(2));
}

#[test]
fn test_count_businesses_in_category_counts_matches() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G1"),
        &String::from_str(&env, "One"),
    );
    client.register_business(
        &String::from_str(&env, "G2"),
        &String::from_str(&env, "Two"),
    );
    client.set_category(&0, &Symbol::new(&env, "retail"));
    client.set_category(&1, &Symbol::new(&env, "fintech"));

    assert_eq!(
        client.count_businesses_in_category(&Symbol::new(&env, "retail")),
        1
    );
}

#[test]
fn test_list_business_ids_in_category_returns_matching_ids() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G1"),
        &String::from_str(&env, "One"),
    );
    client.register_business(
        &String::from_str(&env, "G2"),
        &String::from_str(&env, "Two"),
    );
    client.set_category(&0, &Symbol::new(&env, "retail"));
    client.set_category(&1, &Symbol::new(&env, "retail"));

    let ids = client.list_business_ids_in_category(&Symbol::new(&env, "retail"));
    assert_eq!(ids.len(), 2);
}

#[test]
fn test_get_tier_summary_aggregates_count_and_ids() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G1"),
        &String::from_str(&env, "One"),
    );
    client.register_business(
        &String::from_str(&env, "G2"),
        &String::from_str(&env, "Two"),
    );
    client.set_verification_tier(&0, &3);
    client.set_verification_tier(&1, &3);

    let summary = client.get_tier_summary(&3);
    assert_eq!(summary.tier, 3);
    assert_eq!(summary.business_count, 2);
    assert_eq!(summary.business_ids.len(), 2);
}

#[test]
fn test_get_tier_summary_empty_for_a_tier_with_no_businesses() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    let summary = client.get_tier_summary(&5);
    assert_eq!(summary.tier, 5);
    assert_eq!(summary.business_count, 0);
    assert_eq!(summary.business_ids.len(), 0);
}

#[test]
fn test_initialize_stores_authority_once() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = GeneratedClient::new(&env, &contract_id);
    let authority = Address::generate(&env);
    env.mock_all_auths();

    assert_eq!(client.get_authority(), None);
    client.initialize(&authority);
    assert_eq!(client.get_authority(), Some(authority));
}

#[test]
fn test_reinitialization_cannot_replace_authority() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = GeneratedClient::new(&env, &contract_id);
    let authority = Address::generate(&env);
    let replacement = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&authority);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.initialize(&replacement);
    }));
    assert!(result.is_err());
    assert_eq!(client.get_authority(), Some(authority));
}

#[test]
fn test_mutation_before_initialization_fails_without_storage_change() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = GeneratedClient::new(&env, &contract_id);
    let caller = Address::generate(&env);
    env.mock_all_auths();

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.register_business(
            &caller,
            &String::from_str(&env, "GABC..."),
            &String::from_str(&env, "Uninitialized"),
        );
    }));
    assert!(result.is_err());
    assert_eq!(client.count_businesses(), 0);
}

#[test]
fn test_unauthorized_caller_fails_before_registration() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = GeneratedClient::new(&env, &contract_id);
    let authority = Address::generate(&env);
    let attacker = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&authority);

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.register_business(
            &attacker,
            &String::from_str(&env, "GATTACK"),
            &String::from_str(&env, "Unauthorized"),
        );
    }));
    assert!(result.is_err());
    assert_eq!(client.count_businesses(), 0);
    assert_eq!(client.get_business(&0), None);
}

#[test]
fn test_unauthorized_caller_cannot_record_signal_or_change_score() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = GeneratedClient::new(&env, &contract_id);
    let authority = Address::generate(&env);
    let attacker = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&authority);
    client.register_business(
        &authority,
        &String::from_str(&env, "GABC..."),
        &String::from_str(&env, "Protected"),
    );

    let signal_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.record_signal(&attacker, &0, &Symbol::new(&env, "payment"), &100);
    }));
    assert!(signal_result.is_err());
    assert_eq!(client.count_signals_for_business(&0), 0);

    let score_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.update_trust_score(&attacker, &0);
    }));
    assert!(score_result.is_err());
    assert_eq!(client.verify_trust_score(&0), 0);
}

#[test]
fn test_unauthorized_caller_cannot_change_profile_state() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = GeneratedClient::new(&env, &contract_id);
    let authority = Address::generate(&env);
    let attacker = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&authority);
    client.set_profile(&authority, &7, &Symbol::new(&env, "logistics"), &2, &true);

    let category_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.set_category(&attacker, &7, &Symbol::new(&env, "finance"));
    }));
    assert!(category_result.is_err());
    assert_eq!(client.get_category(&7), Symbol::new(&env, "logistics"));

    let tier_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.set_verification_tier(&attacker, &7, &9);
    }));
    assert!(tier_result.is_err());
    assert_eq!(client.get_verification_tier(&7), 2);

    let active_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.deactivate_business(&attacker, &7);
    }));
    assert!(active_result.is_err());
    assert!(client.is_active(&7));
}

#[test]
fn test_authorized_composite_and_tier_mutations_use_same_authority() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = GeneratedClient::new(&env, &contract_id);
    let authority = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&authority);

    let id = client.register_verified_business(
        &authority,
        &String::from_str(&env, "GABC..."),
        &String::from_str(&env, "Authorized"),
        &2,
    );
    assert_eq!(client.bump_tier(&authority, &id), 3);
    assert_eq!(client.downgrade_tier(&authority, &id), 2);
    client.set_profile(&authority, &id, &Symbol::new(&env, "finance"), &4, &false);
    assert_eq!(client.get_category(&id), Symbol::new(&env, "finance"));
    assert_eq!(client.get_verification_tier(&id), 4);
    assert!(!client.is_active(&id));
}

#[test]
fn test_pagination() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TrustLayerContract);
    let client = GeneratedClient::new(&env, &contract_id);
    let authority = Address::generate(&env);
    env.mock_all_auths();
    client.initialize(&authority);
    
    // Add a large fixture
    for i in 0..10 {
        let wallet = String::from_str(&env, &std::format!("G100{}", i));
        let name = String::from_str(&env, "Test");
        let id = client.register_business(&authority, &wallet, &name);
        client.set_verification_tier(&authority, &id, &(if i % 2 == 0 { 1 } else { 2 }));
        client.set_category(&authority, &id, &Symbol::new(&env, "cat1"));
        client.record_signal(&authority, &id, &Symbol::new(&env, "payment"), &(100 + i as i128));
    }
    
    // invalid-limit
    let err = client.try_get_businesses_paged(&None, &101).unwrap_err();
    assert_eq!(err.unwrap(), super::QueryError::LimitExceeded);
    
    // one-page
    let page1 = client.get_businesses_paged(&None, &10);
    assert_eq!(page1.business_ids.len(), 10);
    assert_eq!(page1.next_cursor, None);
    
    // multi-page
    let page1 = client.get_businesses_paged(&None, &4);
    assert_eq!(page1.business_ids.len(), 4);
    assert_eq!(page1.next_cursor, Some(4));
    
    let page2 = client.get_businesses_paged(&Some(4), &4);
    assert_eq!(page2.business_ids.len(), 4);
    assert_eq!(page2.next_cursor, Some(8));
    
    let page3 = client.get_businesses_paged(&Some(8), &4);
    assert_eq!(page3.business_ids.len(), 2);
    assert_eq!(page3.next_cursor, None);
    
    // empty
    let empty = client.get_businesses_paged(&Some(10), &4);
    assert_eq!(empty.business_ids.len(), 0);
    assert_eq!(empty.next_cursor, None);
    
    // invalid cursor
    let err = client.try_get_businesses_paged(&Some(11), &4).unwrap_err();
    assert_eq!(err.unwrap(), super::QueryError::InvalidCursor);
    
    // signals
    let sig_page = client.get_signals_for_business_paged(&0, &None, &5);
    assert_eq!(sig_page.signals.len(), 1);
    
    // filtered pages match count
    let tier_page = client.get_businesses_at_tier_paged(&1, &None, &10);
    assert_eq!(tier_page.business_ids.len(), 5);
}
