#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    Admin,
    Credits(Address),
    Votes(u32), // poll_id -> total_votes
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract is already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    pub fn mint_credits(env: Env, member: Address, amount: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let key = DataKey::Credits(member.clone());
        let current_credits: u32 = env.storage().persistent().get(&key).unwrap_or(0);
        env.storage().persistent().set(&key, &(current_credits + amount));
    }

    pub fn vote(env: Env, voter: Address, poll_id: u32, num_votes: u32) {
        voter.require_auth();

        let credit_key = DataKey::Credits(voter.clone());
        let current_credits: u32 = env.storage().persistent().get(&credit_key).unwrap_or(0);
        
        // Cơ chế Quadratic Voting: Chi phí = Bình phương số phiếu
        let cost = num_votes * num_votes; 
        
        if current_credits < cost {
            panic!("Insufficient voting credits");
        }

        env.storage().persistent().set(&credit_key, &(current_credits - cost));

        let vote_key = DataKey::Votes(poll_id);
        let current_votes: u32 = env.storage().persistent().get(&vote_key).unwrap_or(0);
        env.storage().persistent().set(&vote_key, &(current_votes + num_votes));
    }

    pub fn get_credits(env: Env, member: Address) -> u32 {
        env.storage().persistent().get(&DataKey::Credits(member)).unwrap_or(0)
    }

    pub fn get_poll_votes(env: Env, poll_id: u32) -> u32 {
        env.storage().persistent().get(&DataKey::Votes(poll_id)).unwrap_or(0)
    }
}

#[cfg(test)]
mod test;