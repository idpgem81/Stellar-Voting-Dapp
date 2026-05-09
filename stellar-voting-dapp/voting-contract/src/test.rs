#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_voting_flow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, VotingContract);
    let client = VotingContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let voter = Address::generate(&env);
    env.mock_all_auths();

    client.initialize(&admin);

    client.mint_credits(&voter, &100);
    assert_eq!(client.get_credits(&voter), 100);

    // Bỏ 4 phiếu, chi phí sẽ là 16 credits
    client.vote(&voter, &1, &4);
    
    assert_eq!(client.get_credits(&voter), 84); // 100 - 16
    assert_eq!(client.get_poll_votes(&1), 4);
}