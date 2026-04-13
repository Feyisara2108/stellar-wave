#![cfg(test)]
extern crate std;

use soroban_sdk::testutils::{Address as _, Events, Ledger};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::{token, Address, Env};

use crate::{WaveContract, WaveContractClient, WaveError};

fn setup() -> (Env, WaveContractClient<'static>, Address, Address, Address, TokenClient<'static>) {
    let env = Env::default();
    let contract_id = env.register(WaveContract, ());
    let client = WaveContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);
    
    // Create token
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_client = TokenClient::new(&env, &token_id.address());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id.address());

    env.mock_all_auths();
    // Mint to sender
    token_admin_client.mint(&sender, &10_000);

    (env, client, admin, sender, recipient, token_client)
}

#[test]
fn init_and_create_stream_success() {
    let (env, client, admin, sender, recipient, token) = setup();
    env.mock_all_auths();

    client.init(&admin);
    let start = 100;
    let cliff = 150;
    let end = 200;
    
    client.create_stream(&1, &sender, &recipient, &token.address, &1_000, &start, &cliff, &end);

    let stream = client.get_stream(&1);
    assert_eq!(stream.total_amount, 1_000);
    assert_eq!(stream.claimed_amount, 0);
    assert_eq!(stream.cliff_ledger, 150);
    assert_eq!(token.balance(&client.address), 1_000);
    assert_eq!(token.balance(&sender), 9_000);
}

#[test]
fn create_stream_rejects_invalid_schedule() {
    let (env, client, admin, sender, recipient, token) = setup();
    env.mock_all_auths();

    client.init(&admin);
    let err = client
        .try_create_stream(&7, &sender, &recipient, &token.address, &10, &100, &50, &90)
        .unwrap_err();
    assert_eq!(err, Ok(WaveError::InvalidSchedule));
}

#[test]
fn claim_with_cliff() {
    let (env, client, admin, sender, recipient, token) = setup();
    env.mock_all_auths();

    client.init(&admin);
    client.create_stream(&9, &sender, &recipient, &token.address, &1_000, &100, &150, &200);

    env.ledger().set_sequence_number(120);
    // Before cliff, nothing to claim
    let err = client.try_claim(&9, &recipient).unwrap_err();
    assert_eq!(err, Ok(WaveError::NothingToClaim));

    // After cliff
    env.ledger().set_sequence_number(150);
    let first = client.claim(&9, &recipient);
    assert_eq!(first, 500); // Half of stream duration passed (100 to 150 of 100 to 200)

    env.ledger().set_sequence_number(250);
    let second = client.claim(&9, &recipient);
    assert_eq!(second, 500);

    let err2 = client.try_claim(&9, &recipient).unwrap_err();
    assert_eq!(err2, Ok(WaveError::NothingToClaim));
}

#[test]
fn cancel_stream() {
    let (env, client, admin, sender, recipient, token) = setup();
    env.mock_all_auths();

    client.init(&admin);
    client.create_stream(&1, &sender, &recipient, &token.address, &1_000, &100, &100, &200);

    env.ledger().set_sequence_number(150); // 500 vested
    client.cancel_stream(&1, &sender);
    
    let stream = client.get_stream(&1);
    assert!(stream.canceled);
    assert_eq!(stream.claimed_amount, 500);
    
    // Check balances
    // Recipient received vested directly at cancel time
    assert_eq!(token.balance(&recipient), 500);
    // Sender received unvested back
    assert_eq!(token.balance(&sender), 9_500); // 9000 left + 500 unvested back

    let err = client.try_claim(&1, &recipient).unwrap_err();
    assert_eq!(err, Ok(WaveError::NothingToClaim));
}

#[test]
fn canceled_already_claimed() {
    let (env, client, admin, sender, recipient, token) = setup();
    env.mock_all_auths();

    client.init(&admin);
    client.create_stream(&1, &sender, &recipient, &token.address, &1_000, &100, &100, &200);

    env.ledger().set_sequence_number(150); // 500 vested
    let first = client.claim(&1, &recipient);
    assert_eq!(first, 500);

    client.cancel_stream(&1, &sender); // Sender cancels, vested is already claimed, so push 0, send 500 back to sender
    assert_eq!(token.balance(&recipient), 500);
    assert_eq!(token.balance(&sender), 9_500);
}

#[test]
fn paused_contract_blocks_state_changes() {
    let (env, client, admin, sender, recipient, token) = setup();
    env.mock_all_auths();

    client.init(&admin);
    client.create_stream(&3, &sender, &recipient, &token.address, &500, &100, &100, &200);
    client.set_paused(&true);

    let err = client.try_claim(&3, &recipient).unwrap_err();
    assert_eq!(err, Ok(WaveError::ContractPaused));

    let err2 = client.try_cancel_stream(&3, &sender).unwrap_err();
    assert_eq!(err2, Ok(WaveError::ContractPaused));
}

#[test]
fn unauthorized_cancel() {
    let (env, client, admin, sender, recipient, token) = setup();
    let attacker = Address::generate(&env);
    env.mock_all_auths();

    client.init(&admin);
    client.create_stream(&4, &sender, &recipient, &token.address, &500, &100, &100, &200);

    let err = client.try_cancel_stream(&4, &attacker).unwrap_err();
    assert_eq!(err, Ok(WaveError::Unauthorized));
}

#[test]
fn admin_cancel() {
    let (env, client, admin, sender, recipient, token) = setup();
    env.mock_all_auths();

    client.init(&admin);
    client.create_stream(&4, &sender, &recipient, &token.address, &500, &100, &100, &200);

    client.cancel_stream(&4, &admin); // admin can cancel
    let stream = client.get_stream(&4);
    assert!(stream.canceled);
}

#[test]
fn events_emission() {
    let (mut env, client, admin, sender, recipient, token) = setup();
    env.mock_all_auths();

    client.init(&admin);
    client.create_stream(&1, &sender, &recipient, &token.address, &1_000, &100, &100, &200);

    let events = env.events().all();
    assert!(events.len() > 0);
}
