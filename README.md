# Voting Canister

This is a voting canister for the Internet Computer (IC) built in Rust. It allows users to:
- Add voting options
- Vote for existing options
- Retrieve current vote counts
- Reset poll data

## Setup Instructions

### Prerequisites
- [Rust](https://www.rust-lang.org/)
- [DFX SDK](https://internetcomputer.org/docs/current/developer-docs/quickstart/quickstart-local/)

### Steps
1. **Clone the Repository**:
   ```bash
   git clone https://github.com/Wambita/voting.git
   cd voting_canister
   ```

2. **Start the DFX Network**:
   ```bash
   dfx start --background
   ```

3. **Deploy the Canister**:
   ```bash
   dfx deploy
   ```

## Usage

- **Add an Option**:
   ```bash
   dfx canister call voting_canister add_option '("Option 1")'
   ```

- **Vote for an Option**:
   ```bash
   dfx canister call voting_canister vote '("Option 1")'
   ```

- **Get Poll Results**:
   ```bash
   dfx canister call voting_canister get_results
   ```

- **Reset the Poll**:
   ```bash
   dfx canister call voting_canister reset_poll
   ```

## License
This project is licensed under the MIT License.
