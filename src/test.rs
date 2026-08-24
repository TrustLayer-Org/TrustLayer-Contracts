#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, Symbol};
use soroban_sdk::testutils::Address as _;

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
fn test_tier_admin_initialization_and_readback() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();

    assert_eq!(client.get_tier_admin(), None);
    client.initialize_tier_admin(&admin);
    assert_eq!(client.get_tier_admin(), Some(admin));
}

#[test]
#[should_panic(expected = "tier admin already initialized")]
fn test_tier_admin_cannot_be_reinitialized() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let first = soroban_sdk::Address::generate(&env);
    let second = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();

    client.initialize_tier_admin(&first);
    client.initialize_tier_admin(&second);
}

#[test]
fn test_authorized_set_records_transition_at_lower_boundary() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);

    let change = client.set_tier_authorized(&admin, &7, &0);
    assert_eq!(change.business_id, 7);
    assert_eq!(change.previous_tier, 0);
    assert_eq!(change.next_tier, 0);
    assert_eq!(client.get_verification_tier(&7), 0);
}

#[test]
fn test_authorized_set_accepts_documented_upper_boundary() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);

    let change = client.set_tier_authorized(&admin, &3, &MAX_VERIFICATION_TIER);
    assert_eq!(change.next_tier, MAX_VERIFICATION_TIER);
    assert_eq!(client.meets_tier(&3, &MAX_VERIFICATION_TIER), true);
}

#[test]
#[should_panic(expected = "verification tier exceeds maximum")]
fn test_authorized_set_rejects_upper_boundary_overflow() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);
    client.set_tier_authorized(&admin, &3, &(MAX_VERIFICATION_TIER + 1));
}

#[test]
#[should_panic(expected = "verification tier is already at maximum")]
fn test_authorized_bump_rejects_overflow_at_maximum() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);
    client.set_tier_authorized(&admin, &3, &MAX_VERIFICATION_TIER);
    client.bump_tier_authorized(&admin, &3);
}

#[test]
fn test_authorized_bump_is_checked_and_repeated() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);

    for expected in 1..=MAX_VERIFICATION_TIER {
        let change = client.bump_tier_authorized(&admin, &4);
        assert_eq!(change.next_tier, expected);
        assert_eq!(client.get_verification_tier(&4), expected);
    }
}

#[test]
fn test_authorized_downgrade_floors_without_underflow() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);

    let first = client.downgrade_tier_authorized(&admin, &11);
    let second = client.downgrade_tier_authorized(&admin, &11);
    assert_eq!(first.previous_tier, 0);
    assert_eq!(first.next_tier, 0);
    assert_eq!(second.next_tier, 0);
}

#[test]
#[should_panic(expected = "unauthorized tier administrator")]
fn test_wrong_admin_cannot_set_tier() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    let attacker = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);
    client.set_tier_authorized(&attacker, &1, &9);
}

#[test]
#[should_panic(expected = "unauthorized tier administrator")]
fn test_wrong_admin_cannot_bump_tier() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    let attacker = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);
    client.bump_tier_authorized(&attacker, &1);
}

#[test]
#[should_panic(expected = "unauthorized tier administrator")]
fn test_wrong_admin_cannot_downgrade_tier() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    let attacker = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);
    client.downgrade_tier_authorized(&attacker, &1);
}

#[test]
fn test_authorized_profile_update_is_observed_as_one_complete_view() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);

    let profile = client.set_profile_authorized(
        &admin,
        &8,
        &Symbol::new(&env, "finance"),
        &MAX_VERIFICATION_TIER,
        &false,
    );
    assert_eq!(profile.business_id, 8);
    assert_eq!(profile.category, Symbol::new(&env, "finance"));
    assert_eq!(profile.tier, MAX_VERIFICATION_TIER);
    assert_eq!(profile.active, false);
    let observed = client.get_profile(&8);
    assert_eq!(observed, profile);
}

#[test]
#[should_panic(expected = "unauthorized tier administrator")]
fn test_wrong_admin_cannot_update_compound_profile() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    let attacker = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);
    client.set_profile_authorized(&attacker, &8, &Symbol::new(&env, "finance"), &9, &true);
}

#[test]
fn test_legacy_writes_become_authorized_after_policy_initialization() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);
    client.set_verification_tier(&6, &4);
    client.set_profile(&6, &Symbol::new(&env, "banking"), &5, &true);
    assert_eq!(client.get_verification_tier(&6), 5);
    assert_eq!(client.get_category(&6), Symbol::new(&env, "banking"));
}

#[test]
#[should_panic(expected = "verification tier exceeds maximum")]
fn test_legacy_write_rejects_invalid_tier_after_policy_initialization() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    env.mock_all_auths();
    client.initialize_tier_admin(&admin);
    client.set_verification_tier(&6, &(MAX_VERIFICATION_TIER + 1));
}
