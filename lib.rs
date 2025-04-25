#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Map, Vec, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Proposal {
    pub id: u32,
    pub description: Symbol,
    pub votes_for: u32,
    pub votes_against: u32,
    pub active: bool,
}

#[contract]
pub struct BasicDaoVoting;

const PROPOSAL_COUNTER: Symbol = symbol_short!("P_COUNT");
const PROPOSALS: Symbol = symbol_short!("PROPOSALS");

#[contractimpl]
impl BasicDaoVoting {
    pub fn create_proposal(env: Env, description: Symbol) -> u32 {
        let mut counter: u32 = env.storage().instance().get(&PROPOSAL_COUNTER).unwrap_or(0);
        counter += 1;

        let proposal = Proposal {
            id: counter,
            description,
            votes_for: 0,
            votes_against: 0,
            active: true,
        };

        let mut proposals: Map<u32, Proposal> = env
            .storage()
            .instance()
            .get(&PROPOSALS)
            .unwrap_or(Map::new(&env));
        proposals.set(counter, proposal);
        env.storage().instance().set(&PROPOSALS, &proposals);
        env.storage().instance().set(&PROPOSAL_COUNTER, &counter);

        counter
    }

    pub fn vote(env: Env, proposal_id: u32, support: bool) {
        let mut proposals: Map<u32, Proposal> = env
            .storage()
            .instance()
            .get(&PROPOSALS)
            .unwrap_or(Map::new(&env));

        if let Some(mut proposal) = proposals.get(proposal_id) {
            if !proposal.active {
                panic!("Proposal is no longer active");
            }

            if support {
                proposal.votes_for += 1;
            } else {
                proposal.votes_against += 1;
            }

            proposals.set(proposal_id, proposal);
            env.storage().instance().set(&PROPOSALS, &proposals);
        } else {
            panic!("Proposal not found");
        }
    }

    pub fn close_proposal(env: Env, proposal_id: u32) {
        let mut proposals: Map<u32, Proposal> = env
            .storage()
            .instance()
            .get(&PROPOSALS)
            .unwrap_or(Map::new(&env));

        if let Some(mut proposal) = proposals.get(proposal_id) {
            proposal.active = false;
            proposals.set(proposal_id, proposal);
            env.storage().instance().set(&PROPOSALS, &proposals);
        } else {
            panic!("Proposal not found");
        }
    }

    pub fn get_proposal(env: Env, proposal_id: u32) -> Proposal {
        let proposals: Map<u32, Proposal> = env
            .storage()
            .instance()
            .get(&PROPOSALS)
            .unwrap_or(Map::new(&env));

        proposals.get(proposal_id).unwrap_or_else(|| panic!("Proposal not found"))
    }
}
