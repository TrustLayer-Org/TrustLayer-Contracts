#![cfg(test)]

extern crate std;

use super::*;
use soroban_sdk::{Env, String, Symbol};

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
fn test_signal_schema_is_versioned_and_deterministic() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    let schema = client.get_signal_schema();
    assert_eq!(schema.version, 1);
    assert_eq!(schema.min_value, -1_000_000);
    assert_eq!(schema.max_value, 1_000_000);
    assert_eq!(schema.max_type_len, 16);
    assert_eq!(schema.allowed_types.len(), 5);
    assert_eq!(
        schema.allowed_types.get(0),
        Some(Symbol::new(&env, "payment"))
    );
    assert_eq!(
        schema.allowed_types.get(1),
        Some(Symbol::new(&env, "review"))
    );
    assert_eq!(
        schema.allowed_types.get(2),
        Some(Symbol::new(&env, "delivery"))
    );
    assert_eq!(
        schema.allowed_types.get(3),
        Some(Symbol::new(&env, "compliance"))
    );
    assert_eq!(
        schema.allowed_types.get(4),
        Some(Symbol::new(&env, "dispute"))
    );
    assert_eq!(schema, client.get_signal_schema());
}

#[test]
fn test_every_supported_signal_type_is_accepted() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let supported = ["payment", "review", "delivery", "compliance", "dispute"];

    for (index, signal_type) in supported.iter().enumerate() {
        assert!(client.record_signal(&0, &Symbol::new(&env, signal_type), &(index as i128),));
    }
    assert_eq!(
        client.count_signals_for_business(&0),
        supported.len() as u32
    );
}

#[test]
fn test_signal_value_inclusive_boundaries_are_accepted() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let signal_type = Symbol::new(&env, "payment");

    assert!(client.record_signal(&0, &signal_type, &-1_000_000));
    assert!(client.record_signal(&0, &signal_type, &0));
    assert!(client.record_signal(&0, &signal_type, &1_000_000));
    assert_eq!(client.count_signals_for_business(&0), 3);
    assert_eq!(client.latest_signal_value(&0), Some(1_000_000));
}

#[test]
fn test_unknown_and_empty_signal_types_fail_before_storage_write() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    let unknown = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.record_signal(&0, &Symbol::new(&env, "unknown"), &10);
    }));
    assert!(unknown.is_err());
    assert_eq!(client.count_signals_for_business(&0), 0);

    let empty = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.record_signal(&0, &Symbol::new(&env, ""), &10);
    }));
    assert!(empty.is_err());
    assert_eq!(client.count_signals_for_business(&0), 0);
    assert_eq!(client.verify_trust_score(&0), 0);
}

#[test]
fn test_oversized_signal_type_fails_before_storage_write() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let oversized = Symbol::new(&env, "signal_type_too_long");

    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.record_signal(&0, &oversized, &10);
    }));
    assert!(result.is_err());
    assert_eq!(client.count_signals_for_business(&0), 0);
}

#[test]
fn test_values_outside_inclusive_range_fail_without_score_changes() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let signal_type = Symbol::new(&env, "review");

    client.record_signal(&0, &signal_type, &100);
    let below_minimum = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.record_signal(&0, &signal_type, &-1_000_001);
    }));
    assert!(below_minimum.is_err());

    let above_maximum = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.record_signal(&0, &signal_type, &1_000_001);
    }));
    assert!(above_maximum.is_err());
    assert_eq!(client.count_signals_for_business(&0), 1);
    assert_eq!(client.verify_trust_score(&0), 0);
    assert_eq!(client.average_signal_value(&0), 100);
}

#[test]
fn test_negative_values_are_scored_without_sign_conversion() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let signal_type = Symbol::new(&env, "compliance");

    client.record_signal(&0, &signal_type, &-100);
    client.record_signal(&0, &signal_type, &100);
    assert_eq!(client.update_trust_score(&0), 0);
    assert_eq!(client.average_signal_value(&0), 0);
}

fn assert_rejected_signal(
    env: &Env,
    client: &TrustLayerContractClient,
    business_id: u32,
    signal_type: &str,
    value: i128,
) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        client.record_signal(&business_id, &Symbol::new(env, signal_type), &value);
    }));
    assert!(result.is_err(), "signal {signal_type}={value} was accepted");
}

#[test]
fn test_schema_type_order_is_stable_for_off_chain_clients() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let schema = client.get_signal_schema();
    let expected = ["payment", "review", "delivery", "compliance", "dispute"];

    for (index, name) in expected.iter().enumerate() {
        assert_eq!(
            schema.allowed_types.get(index as u32),
            Some(Symbol::new(&env, name))
        );
    }
    assert_eq!(schema.allowed_types.get(expected.len() as u32), None);
}

#[test]
fn test_schema_values_are_stable_across_repeated_reads() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    let first = client.get_signal_schema();
    let second = client.get_signal_schema();
    let third = client.get_signal_schema();
    assert_eq!(first.version, second.version);
    assert_eq!(second.version, third.version);
    assert_eq!(first.min_value, second.min_value);
    assert_eq!(second.min_value, third.min_value);
    assert_eq!(first.max_value, second.max_value);
    assert_eq!(second.max_value, third.max_value);
    assert_eq!(first.max_type_len, second.max_type_len);
    assert_eq!(second.max_type_len, third.max_type_len);
    assert_eq!(first.allowed_types, second.allowed_types);
    assert_eq!(second.allowed_types, third.allowed_types);
}

#[test]
fn test_each_supported_type_accepts_minimum_zero_and_maximum() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let types = ["payment", "review", "delivery", "compliance", "dispute"];

    for (business_id, signal_type) in types.iter().enumerate() {
        let id = business_id as u32;
        assert!(client.record_signal(&id, &Symbol::new(&env, signal_type), &-1_000_000));
        assert!(client.record_signal(&id, &Symbol::new(&env, signal_type), &0));
        assert!(client.record_signal(&id, &Symbol::new(&env, signal_type), &1_000_000));
        assert_eq!(client.count_signals_for_business(&id), 3);
        assert_eq!(client.latest_signal_value(&id), Some(1_000_000));
        assert_eq!(
            client.signal_type_count(&id, &Symbol::new(&env, signal_type)),
            3
        );
    }
}

#[test]
fn test_each_supported_type_rejects_value_just_beyond_both_bounds() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let types = ["payment", "review", "delivery", "compliance", "dispute"];

    for (business_id, signal_type) in types.iter().enumerate() {
        let id = business_id as u32;
        assert_rejected_signal(&env, &client, id, signal_type, -1_000_001);
        assert_rejected_signal(&env, &client, id, signal_type, 1_000_001);
        assert_eq!(client.count_signals_for_business(&id), 0);
        assert_eq!(client.latest_signal_value(&id), None);
    }
}

#[test]
fn test_i128_extremes_are_rejected_without_arithmetic_overflow() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_rejected_signal(&env, &client, 0, "payment", i128::MIN);
    assert_rejected_signal(&env, &client, 0, "payment", i128::MAX);
    assert_rejected_signal(&env, &client, 0, "review", i128::MIN + 1);
    assert_rejected_signal(&env, &client, 0, "review", i128::MAX - 1);
    assert_eq!(client.count_signals_for_business(&0), 0);
}

#[test]
fn test_unknown_symbol_families_are_all_rejected() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let unknown_types = [
        "",
        "unknown",
        "Payment",
        "payment_v2",
        "payments",
        "reviewer",
        "oracle",
        "score",
        "admin",
        "test",
    ];

    for (index, signal_type) in unknown_types.iter().enumerate() {
        assert_rejected_signal(&env, &client, index as u32, signal_type, 1);
        assert_eq!(client.count_signals_for_business(&(index as u32)), 0);
    }
}

#[test]
fn test_oversized_symbols_are_rejected_at_metadata_boundary() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let oversized_types = [
        "12345678901234567",
        "signal_type_too_long",
        "this_symbol_is_far_too_long_for_v1",
    ];

    for (index, signal_type) in oversized_types.iter().enumerate() {
        assert!(signal_type.len() > 16);
        assert_rejected_signal(&env, &client, index as u32, signal_type, 1);
        assert_eq!(client.count_signals_for_business(&(index as u32)), 0);
    }
}

#[test]
fn test_sixteen_character_unknown_symbol_is_bounded_but_not_allowlisted() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let bounded_unknown = "1234567890123456";
    assert_eq!(bounded_unknown.len(), 16);

    assert_rejected_signal(&env, &client, 0, bounded_unknown, 1);
    assert_eq!(client.count_signals_for_business(&0), 0);
}

#[test]
fn test_invalid_symbols_cannot_change_existing_signal_statistics() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let payment = Symbol::new(&env, "payment");
    client.record_signal(&0, &payment, &10);
    client.record_signal(&0, &payment, &20);
    client.record_signal(&0, &payment, &30);

    let before_count = client.count_signals_for_business(&0);
    let before_latest = client.latest_signal_value(&0);
    let before_average = client.average_signal_value(&0);
    let before_payment_count = client.signal_type_count(&0, &payment);
    assert_rejected_signal(&env, &client, 0, "unknown", 99);
    assert_rejected_signal(&env, &client, 0, "", 99);
    assert_rejected_signal(&env, &client, "payment".len() as u32, "delivery", i128::MAX);
    assert_rejected_signal(&env, &client, 0, "signal_type_too_long", 99);

    assert_eq!(client.count_signals_for_business(&0), before_count);
    assert_eq!(client.latest_signal_value(&0), before_latest);
    assert_eq!(client.average_signal_value(&0), before_average);
    assert_eq!(client.signal_type_count(&0, &payment), before_payment_count);
}

#[test]
fn test_failed_submissions_do_not_create_a_score_record() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    assert_rejected_signal(&env, &client, 42, "unknown", 1);
    assert_eq!(client.verify_trust_score(&42), 0);
    assert_eq!(client.average_signal_value(&42), 0);
    assert_eq!(client.get_business_stats(&42).signal_count, 0);
}

#[test]
fn test_valid_negative_and_positive_values_remain_inclusive_after_rejections() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let signal_type = Symbol::new(&env, "delivery");

    assert_rejected_signal(&env, &client, 0, "delivery", -1_000_001);
    assert!(client.record_signal(&0, &signal_type, &-1_000_000));
    assert_rejected_signal(&env, &client, 0, "delivery", 1_000_001);
    assert!(client.record_signal(&0, &signal_type, &1_000_000));
    assert_eq!(client.count_signals_for_business(&0), 2);
    assert_eq!(client.average_signal_value(&0), 0);
}

#[test]
fn test_schema_bounds_match_the_values_accepted_by_record_signal() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let schema = client.get_signal_schema();
    let signal_type = Symbol::new(&env, "compliance");

    assert!(client.record_signal(&0, &signal_type, &schema.min_value));
    assert!(client.record_signal(&0, &signal_type, &schema.max_value));
    assert_rejected_signal(&env, &client, 0, "compliance", schema.min_value - 1);
    assert_rejected_signal(&env, &client, 0, "compliance", schema.max_value + 1);
    assert_eq!(client.count_signals_for_business(&0), 2);
}

#[test]
fn test_schema_max_type_length_is_reported_as_a_client_contract() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let schema = client.get_signal_schema();

    assert_eq!(schema.max_type_len, 16);
    assert!(schema.max_type_len > 0);
    assert!(schema.max_type_len <= 32);
    for signal_type in schema.allowed_types.iter() {
        assert!(signal_type != Symbol::new(&env, ""));
    }
}

#[test]
fn test_validation_is_independent_per_business() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let payment = Symbol::new(&env, "payment");
    let review = Symbol::new(&env, "review");

    assert!(client.record_signal(&0, &payment, &100));
    assert_rejected_signal(&env, &client, 1, "unknown", 100);
    assert!(client.record_signal(&1, &review, &-100));
    assert_rejected_signal(&env, &client, 0, "payment_v2", 100);

    assert_eq!(client.count_signals_for_business(&0), 1);
    assert_eq!(client.count_signals_for_business(&1), 1);
    assert_eq!(client.latest_signal_value(&0), Some(100));
    assert_eq!(client.latest_signal_value(&1), Some(-100));
}

#[test]
fn test_allowlisted_types_are_counted_independently_after_mixed_values() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let payment = Symbol::new(&env, "payment");
    let review = Symbol::new(&env, "review");
    let delivery = Symbol::new(&env, "delivery");
    let compliance = Symbol::new(&env, "compliance");
    let dispute = Symbol::new(&env, "dispute");

    client.record_signal(&0, &payment, &10);
    client.record_signal(&0, &review, &20);
    client.record_signal(&0, &delivery, &30);
    client.record_signal(&0, &compliance, &40);
    client.record_signal(&0, &dispute, &50);

    assert_eq!(client.signal_type_count(&0, &payment), 1);
    assert_eq!(client.signal_type_count(&0, &review), 1);
    assert_eq!(client.signal_type_count(&0, &delivery), 1);
    assert_eq!(client.signal_type_count(&0, &compliance), 1);
    assert_eq!(client.signal_type_count(&0, &dispute), 1);
    assert_eq!(client.count_signals_for_business(&0), 5);
    assert_eq!(client.average_signal_value(&0), 30);
}
