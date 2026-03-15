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
