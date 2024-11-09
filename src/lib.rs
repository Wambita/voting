use std::collections::HashMap;
use ic_cdk::storage;
use ic_cdk_macros::{init, query, update};

// Define the main data structure
#[derive(Default)]
struct Poll {
    options: HashMap<String, u32>,  // Option name and vote count
}

// Initialize the canister
#[init]
fn init() {
    storage::get_mut::<Poll>().options = HashMap::new();
}

// Function to add an option to the poll
#[update]
fn add_option(option: String) -> String {
    let poll = storage::get_mut::<Poll>();
    if poll.options.contains_key(&option) {
        "Option already exists!".to_string()
    } else {
        poll.options.insert(option.clone(), 0);
        format!("Option '{}' added.", option)
    }
}

// Function to vote for an option
#[update]
fn vote(option: String) -> String {
    let poll = storage::get_mut::<Poll>();
    match poll.options.get_mut(&option) {
        Some(count) => {
            *count += 1;
            format!("Voted for '{}'.", option)
        }
        None => "Option not found!".to_string(),
    }
}

// Function to get the current results of the poll
#[query]
fn get_results() -> HashMap<String, u32> {
    let poll = storage::get::<Poll>();
    poll.options.clone()
}

// Function to reset the poll
#[update]
fn reset_poll() -> String {
    let poll = storage::get_mut::<Poll>();
    poll.options.clear();
    "Poll has been reset.".to_string()
}
