extern crate std;

use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{Address, Env};

use crate::{WaveContract, WaveContractClient, WaveError};

fn setup() -> (Env, WaveContractClient<'static>, Address, Address) {
    let env = Env::default();
    let contract_id = env.register(WaveContract, ());
    let client = WaveContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let recipient = Address::generate(&env);
    (env, client, admin, recipient)
}

#[test]
fn init_and_create_stream_success() {
    let (env, client, admin, recipient) = setup();
    env.mock_all_auths();

    client.init(&admin);
    client.create_stream(&1, &recipient, &1_000, &100, &200);

    let stream = client.get_stream(&1);
    assert_eq!(stream.total_amount, 1_000);
    assert_eq!(stream.claimed_amount, 0);
}

#[test]
fn create_stream_rejects_invalid_schedule() {
    let (env, client, admin, recipient) = setup();
    env.mock_all_auths();

    client.init(&admin);
    let err = client
        .try_create_stream(&7, &recipient, &10, &10, &10)
        .unwrap_err();
    assert_eq!(err, Ok(WaveError::InvalidSchedule));
}

#[test]
fn claim_vests_over_time_and_caps_at_total() {
    let (env, client, admin, recipient) = setup();
    env.mock_all_auths();

    client.init(&admin);
    client.create_stream(&9, &recipient, &1_000, &100, &200);

    env.ledger().set_sequence_number(150);
    let first = client.claim(&9, &recipient);
    assert_eq!(first, 500);

    env.ledger().set_sequence_number(250);
    let second = client.claim(&9, &recipient);
    assert_eq!(second, 500);

    let err = client.try_claim(&9, &recipient).unwrap_err();
    assert_eq!(err, Ok(WaveError::NothingToClaim));
}

#[test]
fn unauthorized_recipient_is_rejected() {
    let (env, client, admin, recipient) = setup();
    let attacker = Address::generate(&env);
    env.mock_all_auths();

    client.init(&admin);
    client.create_stream(&11, &recipient, &100, &100, &200);

    let err = client.try_claim(&11, &attacker).unwrap_err();
    assert_eq!(err, Ok(WaveError::Unauthorized));
}

#[test]
fn paused_contract_blocks_state_changes() {
    let (env, client, admin, recipient) = setup();
    env.mock_all_auths();

    client.init(&admin);
    client.create_stream(&3, &recipient, &500, &100, &200);
    client.set_paused(&true);

    let err = client.try_claim(&3, &recipient).unwrap_err();
    assert_eq!(err, Ok(WaveError::ContractPaused));
}
