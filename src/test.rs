#![cfg(test)]

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
#[should_panic(expected = "inactive business cannot receive trust data")]
fn test_inactive_business_rejects_new_signals() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G-INACTIVE"),
        &String::from_str(&env, "Inactive Business"),
    );
    client.deactivate_business(&0);

    client.record_signal(&0, &Symbol::new(&env, "payment"), &100);
}

#[test]
#[should_panic(expected = "inactive business cannot receive trust data")]
fn test_inactive_business_rejects_score_updates() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G-INACTIVE-SCORE"),
        &String::from_str(&env, "Inactive Score Business"),
    );
    client.record_signal(&0, &Symbol::new(&env, "payment"), &100);
    client.update_trust_score(&0);
    client.deactivate_business(&0);

    client.update_trust_score(&0);
}

#[test]
fn test_score_and_stats_history_survive_deactivation() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G-HISTORY"),
        &String::from_str(&env, "History Business"),
    );
    client.set_category(&0, &Symbol::new(&env, "finance"));
    client.set_verification_tier(&0, &3);
    client.record_signal(&0, &Symbol::new(&env, "payment"), &100);
    client.record_signal(&0, &Symbol::new(&env, "review"), &200);
    assert_eq!(client.update_trust_score(&0), 150);

    client.deactivate_business(&0);

    assert_eq!(client.is_active(&0), false);
    assert_eq!(client.is_verified(&0), true);
    assert_eq!(client.is_active_and_verified(&0), false);
    assert_eq!(client.verify_trust_score(&0), 150);
    assert_eq!(client.count_signals_for_business(&0), 2);
    assert_eq!(client.latest_signal_value(&0), Some(200));
    assert_eq!(client.average_signal_value(&0), 150);
    assert_eq!(
        client.signal_type_count(&0, &Symbol::new(&env, "payment")),
        1
    );
    assert_eq!(
        client.get_business_stats(&0),
        BusinessStats {
            business_id: 0,
            signal_count: 2,
            average_value: 150,
            has_signals: true,
        }
    );
    assert_eq!(
        client.get_profile(&0),
        BusinessProfile {
            business_id: 0,
            category: Symbol::new(&env, "finance"),
            tier: 3,
            active: false,
        }
    );
}

#[test]
fn test_reactivation_restores_writes_without_resetting_history() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G-REACTIVATE"),
        &String::from_str(&env, "Reactivation Business"),
    );
    client.record_signal(&0, &Symbol::new(&env, "payment"), &100);
    assert_eq!(client.update_trust_score(&0), 100);
    client.deactivate_business(&0);
    client.reactivate_business(&0);

    assert!(client.is_active(&0));
    assert_eq!(client.verify_trust_score(&0), 100);
    assert_eq!(client.count_signals_for_business(&0), 1);

    client.record_signal(&0, &Symbol::new(&env, "payment"), &300);
    assert_eq!(client.count_signals_for_business(&0), 2);
    assert_eq!(client.update_trust_score(&0), 200);
    assert_eq!(client.verify_trust_score(&0), 200);
}

#[test]
fn test_directory_queries_exclude_inactive_businesses_consistently() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    for (wallet, company) in [
        ("G-DIR-0", "Directory One"),
        ("G-DIR-1", "Directory Two"),
        ("G-DIR-2", "Directory Three"),
        ("G-DIR-3", "Directory Four"),
    ] {
        client.register_business(
            &String::from_str(&env, wallet),
            &String::from_str(&env, company),
        );
    }
    client.set_category(&0, &Symbol::new(&env, "finance"));
    client.set_verification_tier(&0, &4);
    client.set_category(&1, &Symbol::new(&env, "finance"));
    client.set_verification_tier(&1, &4);
    client.deactivate_business(&1);
    client.set_category(&2, &Symbol::new(&env, "retail"));
    client.set_verification_tier(&2, &2);
    client.set_category(&3, &Symbol::new(&env, "retail"));
    client.set_verification_tier(&3, &9);
    client.deactivate_business(&3);

    assert_eq!(client.count_businesses(), 4);
    assert_eq!(client.count_active_businesses(), 2);
    assert_eq!(client.count_businesses_at_tier(&4), 1);
    assert_eq!(
        client.list_business_ids_at_tier(&4),
        soroban_sdk::vec![&env, 0]
    );
    assert_eq!(client.highest_tier(), 4);
    assert_eq!(
        client.list_business_ids_meeting_tier(&2),
        soroban_sdk::vec![&env, 0, 2]
    );
    assert_eq!(
        client.count_businesses_in_category(&Symbol::new(&env, "finance")),
        1
    );
    assert_eq!(
        client.list_business_ids_in_category(&Symbol::new(&env, "finance")),
        soroban_sdk::vec![&env, 0]
    );
    assert_eq!(
        client.count_businesses_in_category(&Symbol::new(&env, "retail")),
        1
    );
    assert_eq!(
        client.get_tier_summary(&4),
        TierSummary {
            tier: 4,
            business_count: 1,
            business_ids: soroban_sdk::vec![&env, 0],
        }
    );
}

#[test]
fn test_reactivated_business_reappears_in_all_directory_queries() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G-DIR-REACTIVATED"),
        &String::from_str(&env, "Reactivated Directory Business"),
    );
    client.set_category(&0, &Symbol::new(&env, "technology"));
    client.set_verification_tier(&0, &7);
    client.deactivate_business(&0);

    assert_eq!(client.count_active_businesses(), 0);
    assert_eq!(client.count_businesses_at_tier(&7), 0);
    assert_eq!(
        client.count_businesses_in_category(&Symbol::new(&env, "technology")),
        0
    );
    assert_eq!(client.highest_tier(), 0);

    client.reactivate_business(&0);

    assert_eq!(client.count_active_businesses(), 1);
    assert_eq!(client.count_businesses_at_tier(&7), 1);
    assert_eq!(
        client.list_business_ids_at_tier(&7),
        soroban_sdk::vec![&env, 0]
    );
    assert_eq!(client.highest_tier(), 7);
    assert_eq!(
        client.list_business_ids_meeting_tier(&7),
        soroban_sdk::vec![&env, 0]
    );
    assert_eq!(
        client.count_businesses_in_category(&Symbol::new(&env, "technology")),
        1
    );
    assert_eq!(
        client.list_business_ids_in_category(&Symbol::new(&env, "technology")),
        soroban_sdk::vec![&env, 0]
    );
}

#[test]
fn test_repeated_deactivation_and_reactivation_are_idempotent() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G-IDEMPOTENT"),
        &String::from_str(&env, "Idempotent Business"),
    );
    client.set_category(&0, &Symbol::new(&env, "logistics"));
    client.set_verification_tier(&0, &5);
    client.record_signal(&0, &Symbol::new(&env, "payment"), &80);
    assert_eq!(client.update_trust_score(&0), 80);

    client.deactivate_business(&0);
    client.deactivate_business(&0);
    assert!(!client.is_active(&0));
    assert_eq!(client.count_active_businesses(), 0);
    assert_eq!(client.verify_trust_score(&0), 80);
    assert_eq!(client.count_signals_for_business(&0), 1);

    client.reactivate_business(&0);
    client.reactivate_business(&0);
    assert!(client.is_active(&0));
    assert_eq!(client.count_active_businesses(), 1);
    assert_eq!(client.get_verification_tier(&0), 5);
    assert_eq!(client.get_category(&0), Symbol::new(&env, "logistics"));
    assert_eq!(client.verify_trust_score(&0), 80);
}

#[test]
fn test_try_rejected_signal_does_not_change_inactive_history() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G-REJECTED-SIGNAL"),
        &String::from_str(&env, "Rejected Signal Business"),
    );
    client.record_signal(&0, &Symbol::new(&env, "payment"), &25);
    assert_eq!(client.update_trust_score(&0), 25);
    client.deactivate_business(&0);

    let result = client.try_record_signal(&0, &Symbol::new(&env, "blocked"), &999);
    assert!(result.is_err());
    assert_eq!(client.count_signals_for_business(&0), 1);
    assert_eq!(client.latest_signal_value(&0), Some(25));
    assert_eq!(client.average_signal_value(&0), 25);
    assert_eq!(
        client.signal_type_count(&0, &Symbol::new(&env, "blocked")),
        0
    );
    assert_eq!(client.verify_trust_score(&0), 25);
}

#[test]
fn test_try_rejected_score_update_does_not_change_stored_score() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G-REJECTED-SCORE"),
        &String::from_str(&env, "Rejected Score Business"),
    );
    client.record_signal(&0, &Symbol::new(&env, "payment"), &40);
    client.record_signal(&0, &Symbol::new(&env, "payment"), &60);
    assert_eq!(client.update_trust_score(&0), 50);
    client.deactivate_business(&0);

    let result = client.try_update_trust_score(&0);
    assert!(result.is_err());
    assert_eq!(client.verify_trust_score(&0), 50);
    assert_eq!(client.count_signals_for_business(&0), 2);
    assert_eq!(client.get_business_stats(&0).average_value, 50);
}

#[test]
fn test_active_business_can_mutate_all_trust_data_before_deactivation() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G-ACTIVE"),
        &String::from_str(&env, "Active Business"),
    );
    client.set_category(&0, &Symbol::new(&env, "commerce"));
    client.set_verification_tier(&0, &6);
    assert!(client.is_active(&0));
    assert!(client.record_signal(&0, &Symbol::new(&env, "payment"), &125));
    assert!(client.record_signal(&0, &Symbol::new(&env, "review"), &175));
    assert_eq!(client.update_trust_score(&0), 150);
    assert_eq!(client.verify_trust_score(&0), 150);
    assert_eq!(client.count_signals_for_business(&0), 2);
    assert!(client.has_signals(&0));
    assert_eq!(client.latest_signal_value(&0), Some(175));
    assert_eq!(client.average_signal_value(&0), 150);
    assert_eq!(
        client.signal_type_count(&0, &Symbol::new(&env, "payment")),
        1
    );
    assert_eq!(client.count_active_businesses(), 1);
    assert_eq!(client.count_businesses_at_tier(&6), 1);
    assert_eq!(
        client.count_businesses_in_category(&Symbol::new(&env, "commerce")),
        1
    );
}

#[test]
fn test_deactivation_preserves_registry_identity_and_profile_fields() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    let id = client.register_business(
        &String::from_str(&env, "G-IDENTITY"),
        &String::from_str(&env, "Identity Business"),
    );
    client.set_profile(&id, &Symbol::new(&env, "identity"), &8, &true);
    let business_before = client.get_business(&id).unwrap();
    client.deactivate_business(&id);

    assert_eq!(client.count_businesses(), 1);
    assert_eq!(client.get_business(&id), Some(business_before.clone()));
    assert_eq!(
        client.get_profile(&id),
        BusinessProfile {
            business_id: id,
            category: Symbol::new(&env, "identity"),
            tier: 8,
            active: false,
        }
    );
    assert_eq!(client.get_category(&id), Symbol::new(&env, "identity"));
    assert_eq!(client.get_verification_tier(&id), 8);
    assert_eq!(client.is_verified(&id), true);
    assert_eq!(client.is_active_and_verified(&id), false);

    client.reactivate_business(&id);

    assert_eq!(client.get_business(&id), Some(business_before));
    assert_eq!(client.get_category(&id), Symbol::new(&env, "identity"));
    assert_eq!(client.get_verification_tier(&id), 8);
    assert!(client.is_active_and_verified(&id));
}

#[test]
fn test_inactive_tier_zero_is_hidden_from_directory_queries() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G-TIER-ZERO"),
        &String::from_str(&env, "Tier Zero Business"),
    );
    client.set_category(&0, &Symbol::new(&env, "unverified"));
    assert_eq!(client.get_verification_tier(&0), 0);
    client.deactivate_business(&0);

    assert_eq!(client.count_active_businesses(), 0);
    assert_eq!(client.count_businesses_at_tier(&0), 0);
    assert_eq!(client.list_business_ids_at_tier(&0).len(), 0);
    assert_eq!(client.list_business_ids_meeting_tier(&0).len(), 0);
    assert_eq!(
        client.count_businesses_in_category(&Symbol::new(&env, "unverified")),
        0
    );
    assert_eq!(
        client
            .list_business_ids_in_category(&Symbol::new(&env, "unverified"))
            .len(),
        0
    );
    assert_eq!(client.highest_tier(), 0);
    assert_eq!(client.get_tier_summary(&0).business_count, 0);
}

#[test]
fn test_lifecycle_transition_updates_directory_and_history_in_lockstep() {
    let env = Env::default();
    let contract_id = env.register(TrustLayerContract, ());
    let client = TrustLayerContractClient::new(&env, &contract_id);

    client.register_business(
        &String::from_str(&env, "G-TRANSITION-A"),
        &String::from_str(&env, "Transition A"),
    );
    client.register_business(
        &String::from_str(&env, "G-TRANSITION-B"),
        &String::from_str(&env, "Transition B"),
    );
    client.set_category(&0, &Symbol::new(&env, "shared"));
    client.set_verification_tier(&0, &3);
    client.set_category(&1, &Symbol::new(&env, "shared"));
    client.set_verification_tier(&1, &3);
    client.record_signal(&0, &Symbol::new(&env, "payment"), &20);
    client.update_trust_score(&0);
    client.record_signal(&1, &Symbol::new(&env, "payment"), &40);
    client.update_trust_score(&1);

    assert_eq!(client.count_active_businesses(), 2);
    assert_eq!(client.count_businesses_at_tier(&3), 2);
    assert_eq!(
        client.count_businesses_in_category(&Symbol::new(&env, "shared")),
        2
    );
    assert_eq!(client.highest_tier(), 3);
    assert_eq!(client.verify_trust_score(&0), 20);
    assert_eq!(client.verify_trust_score(&1), 40);

    client.deactivate_business(&0);

    assert_eq!(client.count_active_businesses(), 1);
    assert_eq!(client.count_businesses_at_tier(&3), 1);
    assert_eq!(
        client.list_business_ids_at_tier(&3),
        soroban_sdk::vec![&env, 1]
    );
    assert_eq!(
        client.count_businesses_in_category(&Symbol::new(&env, "shared")),
        1
    );
    assert_eq!(
        client.list_business_ids_in_category(&Symbol::new(&env, "shared")),
        soroban_sdk::vec![&env, 1]
    );
    assert_eq!(client.highest_tier(), 3);
    assert_eq!(client.verify_trust_score(&0), 20);
    assert_eq!(client.get_business_stats(&0).signal_count, 1);

    client.reactivate_business(&0);

    assert_eq!(client.count_active_businesses(), 2);
    assert_eq!(client.count_businesses_at_tier(&3), 2);
    assert_eq!(
        client.list_business_ids_at_tier(&3),
        soroban_sdk::vec![&env, 0, 1]
    );
    assert_eq!(
        client.count_businesses_in_category(&Symbol::new(&env, "shared")),
        2
    );
    assert_eq!(
        client.list_business_ids_in_category(&Symbol::new(&env, "shared")),
        soroban_sdk::vec![&env, 0, 1]
    );
    assert_eq!(client.highest_tier(), 3);
    assert_eq!(client.verify_trust_score(&0), 20);
    assert_eq!(client.verify_trust_score(&1), 40);
}
