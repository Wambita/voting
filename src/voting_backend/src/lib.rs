use candid::{CandidType, Deserialize};
use ic_cdk_macros::{query, update};
use std::cell::RefCell;
use std::collections::HashMap;

// Define structures for Candidate and VotingSession
#[derive(CandidType, Deserialize, Clone, Debug)]
struct Candidate {
    id: String,
    name: String,
    votes: u32,
}

#[derive(Default)]
struct VotingSession {
    candidates: HashMap<String, Candidate>,
    voters: HashMap<String, bool>, // To track if a user has already voted
}

// State to hold the voting session
thread_local! {
    static STATE: RefCell<VotingSession> = RefCell::new(VotingSession::default());
}

// Add a new candidate
#[update]
fn add_candidate(id: String, name: String) -> Result<String, String> {
    if id.is_empty() || name.is_empty() {
        return Err("Invalid input: All fields must be non-empty.".to_string());
    }

    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if state.candidates.contains_key(&id) {
            Err(format!("Candidate with ID {} already exists.", id))
        } else {
            state.candidates.insert(id.clone(), Candidate { id: id.clone(), name, votes: 0 });
            Ok(format!("Candidate with ID {} added successfully.", id))
        }
    })
}

// Cast a vote for a candidate
#[update]
fn vote(candidate_id: String, voter_id: String) -> Result<String, String> {
    STATE.with(|state| {
        let mut state = state.borrow_mut();

        // Check if the voter has already voted
        if state.voters.contains_key(&voter_id) {
            return Err(format!("Voter with ID {} has already voted.", voter_id));
        }

        // Find the candidate and increment their vote count
        if let Some(candidate) = state.candidates.get_mut(&candidate_id) {
            candidate.votes += 1;
            state.voters.insert(voter_id, true); // Mark voter as having voted
            Ok(format!("Vote cast for candidate {} successfully.", candidate_id))
        } else {
            Err(format!("Candidate with ID {} not found.", candidate_id))
        }
    })
}

// Retrieve the current vote count for all candidates
#[query]
fn get_results() -> HashMap<String, u32> {
    STATE.with(|state| {
        let state = state.borrow();
        state.candidates.iter().map(|(id, candidate)| (id.clone(), candidate.votes)).collect()
    })
}

// Retrieve candidate information by ID
#[query]
fn get_candidate(candidate_id: String) -> Result<Candidate, String> {
    STATE.with(|state| {
        let state = state.borrow();
        state.candidates.get(&candidate_id).cloned().ok_or_else(|| format!("Candidate with ID {} not found.", candidate_id))
    })
}

// Reset the voting session (e.g., for testing or restarting)
#[update]
fn reset_voting_session() -> String {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.candidates.clear();
        state.voters.clear();
        "Voting session reset successfully.".to_string()
    })
}
