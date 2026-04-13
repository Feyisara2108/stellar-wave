#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Paused,
    Stream(u64),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stream {
    pub token: Address,
    pub recipient: Address,
    pub sender: Address,
    pub total_amount: i128,
    pub claimed_amount: i128,
    pub start_ledger: u32,
    pub cliff_ledger: u32,
    pub end_ledger: u32,
    pub canceled: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamCreatedEvent {
    pub stream_id: u64,
    pub token: Address,
    pub sender: Address,
    pub recipient: Address,
    pub total_amount: i128,
    pub start_ledger: u32,
    pub cliff_ledger: u32,
    pub end_ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamClaimedEvent {
    pub stream_id: u64,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamCanceledEvent {
    pub stream_id: u64,
    pub unvested_amount: i128,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum WaveError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    InvalidSchedule = 5,
    StreamAlreadyExists = 6,
    StreamNotFound = 7,
    NothingToClaim = 8,
    ContractPaused = 9,
    StreamCanceled = 10,
}

#[contract]
pub struct WaveContract;

fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

fn read_admin(env: &Env) -> Result<Address, WaveError> {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(WaveError::NotInitialized)
}

fn read_paused(env: &Env) -> bool {
    env.storage()
        .instance()
        .get(&DataKey::Paused)
        .unwrap_or(false)
}

fn read_stream(env: &Env, id: u64) -> Result<Stream, WaveError> {
    env.storage()
        .persistent()
        .get(&DataKey::Stream(id))
        .ok_or(WaveError::StreamNotFound)
}

fn write_stream(env: &Env, id: u64, stream: &Stream) {
    env.storage().persistent().set(&DataKey::Stream(id), stream);
    env.storage().persistent().extend_ttl(&DataKey::Stream(id), 100000, 100000);
}

fn require_admin(env: &Env) -> Result<Address, WaveError> {
    let admin = read_admin(env)?;
    admin.require_auth();
    Ok(admin)
}

#[contractimpl]
impl WaveContract {
    pub fn init(env: Env, admin: Address) -> Result<(), WaveError> {
        if has_admin(&env) {
            return Err(WaveError::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Paused, &false);
        Ok(())
    }

    pub fn set_paused(env: Env, paused: bool) -> Result<(), WaveError> {
        require_admin(&env)?;
        env.storage().instance().set(&DataKey::Paused, &paused);
        env.events().publish(("stream", "paused"), paused);
        Ok(())
    }

    pub fn create_stream(
        env: Env,
        stream_id: u64,
        sender: Address,
        recipient: Address,
        token: Address,
        total_amount: i128,
        start_ledger: u32,
        cliff_ledger: u32,
        end_ledger: u32,
    ) -> Result<(), WaveError> {
        if read_paused(&env) {
            return Err(WaveError::ContractPaused);
        }
        
        sender.require_auth();

        if env.storage().persistent().has(&DataKey::Stream(stream_id)) {
            return Err(WaveError::StreamAlreadyExists);
        }
        if total_amount <= 0 {
            return Err(WaveError::InvalidAmount);
        }
        // Validate schedule logic
        if start_ledger >= end_ledger || cliff_ledger > end_ledger || cliff_ledger < start_ledger {
            return Err(WaveError::InvalidSchedule);
        }

        // Transfer tokens from sender to contract
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&sender, &env.current_contract_address(), &total_amount);

        let stream = Stream {
            token: token.clone(),
            sender: sender.clone(),
            recipient: recipient.clone(),
            total_amount,
            claimed_amount: 0,
            start_ledger,
            cliff_ledger,
            end_ledger,
            canceled: false,
        };
        write_stream(&env, stream_id, &stream);

        let event = StreamCreatedEvent {
            stream_id,
            token,
            sender,
            recipient,
            total_amount,
            start_ledger,
            cliff_ledger,
            end_ledger,
        };
        env.events().publish(("stream", "created"), event);

        Ok(())
    }

    pub fn claimable(env: Env, stream_id: u64) -> Result<i128, WaveError> {
        let stream = read_stream(&env, stream_id)?;
        Ok(calculate_claimable(&env, &stream))
    }

    pub fn claim(env: Env, stream_id: u64, recipient: Address) -> Result<i128, WaveError> {
        if read_paused(&env) {
            return Err(WaveError::ContractPaused);
        }

        let mut stream = read_stream(&env, stream_id)?;
        
        // Canceled streams can no longer be claimed if there's nothing claimable at cancellation.
        // Actually, let's allow claims of already vested amounts up to point of cancellation?
        // Let's adopt strict behavior: if you didn't claim it before, maybe you still can if it's already vested?
        // Wait, for cancellation, usually the unvested portion goes back to sender, vested can still be claimed or is pushed.
        // Let's push vested to recipient upon cancellation, making it 0 claimable.

        if stream.recipient != recipient {
            return Err(WaveError::Unauthorized);
        }
        recipient.require_auth();

        let claimable = calculate_claimable(&env, &stream);
        if claimable <= 0 {
            return Err(WaveError::NothingToClaim);
        }

        stream.claimed_amount += claimable;
        write_stream(&env, stream_id, &stream);

        // Transfer funds
        let token_client = token::Client::new(&env, &stream.token);
        token_client.transfer(&env.current_contract_address(), &recipient, &claimable);

        env.events().publish(("stream", "claimed"), StreamClaimedEvent { stream_id, amount: claimable });

        Ok(claimable)
    }

    pub fn cancel_stream(env: Env, stream_id: u64, caller: Address) -> Result<(), WaveError> {
        if read_paused(&env) {
            return Err(WaveError::ContractPaused);
        }

        caller.require_auth();

        let mut stream = read_stream(&env, stream_id)?;
        if stream.canceled {
            return Err(WaveError::StreamCanceled);
        }
        // Only sender or admin can cancel
        if stream.sender != caller {
            // Check if caller is admin. If not, it will fail
            let admin = read_admin(&env)?;
            if caller != admin {
                return Err(WaveError::Unauthorized);
            }
        }

        // Send strictly vested to recipient, rest to sender
        let vested = calculate_vested(&env, &stream);
        let unvested = stream.total_amount - vested;

        let claimable = vested.saturating_sub(stream.claimed_amount);
        stream.claimed_amount += claimable;
        stream.canceled = true;

        write_stream(&env, stream_id, &stream);

        let token_client = token::Client::new(&env, &stream.token);
        if claimable > 0 {
            token_client.transfer(&env.current_contract_address(), &stream.recipient, &claimable);
            env.events().publish(("stream", "claimed"), StreamClaimedEvent { stream_id, amount: claimable });
        }
        if unvested > 0 {
            token_client.transfer(&env.current_contract_address(), &stream.sender, &unvested);
        }

        env.events().publish(("stream", "canceled"), StreamCanceledEvent { stream_id, unvested_amount: unvested });

        Ok(())
    }

    pub fn get_stream(env: Env, stream_id: u64) -> Result<Stream, WaveError> {
        read_stream(&env, stream_id)
    }
}

// Calculate total vested up to this point in time
fn calculate_vested(env: &Env, stream: &Stream) -> i128 {
    let now = env.ledger().sequence();
    if now < stream.cliff_ledger {
        0
    } else if now >= stream.end_ledger || stream.canceled {
        // If canceled, we freeze the vested at the time of cancellation, but since we pushed vested at cancel time and set it to claimed, we can just use total_amount or existing logic?
        // Wait, if canceled, the 'unvested' was returned. So the MAX vested for a canceled stream is what they claimed (which is the vested at time of cancellation).
        if stream.canceled {
            stream.claimed_amount
        } else {
            stream.total_amount
        }
    } else {
        let elapsed = (now - stream.start_ledger) as i128;
        let duration = (stream.end_ledger - stream.start_ledger) as i128;
        stream.total_amount.saturating_mul(elapsed) / duration
    }
}

fn calculate_claimable(env: &Env, stream: &Stream) -> i128 {
    let vested = calculate_vested(env, stream);
    vested.saturating_sub(stream.claimed_amount)
}

#[cfg(test)]
mod test;
