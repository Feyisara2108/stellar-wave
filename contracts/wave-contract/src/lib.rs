#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

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
    pub recipient: Address,
    pub total_amount: i128,
    pub claimed_amount: i128,
    pub start_ledger: u32,
    pub end_ledger: u32,
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
        Ok(())
    }

    pub fn create_stream(
        env: Env,
        stream_id: u64,
        recipient: Address,
        total_amount: i128,
        start_ledger: u32,
        end_ledger: u32,
    ) -> Result<(), WaveError> {
        if read_paused(&env) {
            return Err(WaveError::ContractPaused);
        }
        require_admin(&env)?;

        if env.storage().persistent().has(&DataKey::Stream(stream_id)) {
            return Err(WaveError::StreamAlreadyExists);
        }
        if total_amount <= 0 {
            return Err(WaveError::InvalidAmount);
        }
        if start_ledger >= end_ledger {
            return Err(WaveError::InvalidSchedule);
        }

        let stream = Stream {
            recipient,
            total_amount,
            claimed_amount: 0,
            start_ledger,
            end_ledger,
        };
        write_stream(&env, stream_id, &stream);
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
        Ok(claimable)
    }

    pub fn get_stream(env: Env, stream_id: u64) -> Result<Stream, WaveError> {
        read_stream(&env, stream_id)
    }
}

fn calculate_claimable(env: &Env, stream: &Stream) -> i128 {
    let now = env.ledger().sequence();

    let vested = if now <= stream.start_ledger {
        0
    } else if now >= stream.end_ledger {
        stream.total_amount
    } else {
        let elapsed = (now - stream.start_ledger) as i128;
        let duration = (stream.end_ledger - stream.start_ledger) as i128;
        stream.total_amount.saturating_mul(elapsed) / duration
    };

    vested.saturating_sub(stream.claimed_amount)
}

#[cfg(test)]
mod test;
