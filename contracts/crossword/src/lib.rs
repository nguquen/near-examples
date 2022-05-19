// https://www.near-sdk.io/zero-to-hero/basics/overview

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, UnorderedSet},
    env, log, near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, PanicOnDefault, Promise,
};

const PRIZE_AMOUNT: u128 = 5_000_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum AnswerDirection {
    Across,
    Down,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CoordinatePair {
    x: u8,
    y: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Answer {
    num: u8,
    start: CoordinatePair,
    direction: AnswerDirection,
    length: u8,
    clue: String,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum PuzzleStatus {
    Unsolved,
    Solved { memo: String },
}

#[derive(Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct UnsolvedPuzzles {
    puzzles: Vec<JsonPuzzle>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonPuzzle {
    solution_hash: String,
    status: PuzzleStatus,
    answer: Vec<Answer>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Puzzle {
    status: PuzzleStatus,
    answer: Vec<Answer>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Crossword {
    owner_id: AccountId,
    puzzles: LookupMap<String, Puzzle>,
    unsolved_puzzles: UnorderedSet<String>,
}

#[near_bindgen]
impl Crossword {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            puzzles: LookupMap::new(b"c"),
            unsolved_puzzles: UnorderedSet::new(b"u"),
        }
    }

    pub fn new_puzzle(&mut self, solution_hash: String, answers: Vec<Answer>) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the owner may call this method"
        );

        let existing = self.puzzles.insert(
            &solution_hash,
            &Puzzle {
                status: PuzzleStatus::Unsolved,
                answer: answers,
            },
        );

        assert!(existing.is_none(), "Puzzle with that key already exists");
        self.unsolved_puzzles.insert(&solution_hash);
    }

    pub fn submit_solution(&mut self, solution: String, memo: String) {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);

        let mut puzzle = self
            .puzzles
            .get(&hashed_input_hex)
            .expect("ERR_NOT_CORRECT_ANSWER");

        puzzle.status = match puzzle.status {
            PuzzleStatus::Unsolved => PuzzleStatus::Solved { memo: memo.clone() },
            _ => {
                env::panic_str("ERR_PUZZLE_SOLVED");
            }
        };

        self.puzzles.insert(&hashed_input_hex, &puzzle);

        self.unsolved_puzzles.remove(&hashed_input_hex);

        log!(
            "Puzzle with solution hash {} solved, with memo: {}",
            hashed_input_hex,
            memo
        );

        Promise::new(env::predecessor_account_id()).transfer(PRIZE_AMOUNT);
    }

    pub fn get_puzzle_status(&self, solution_hash: String) -> Option<PuzzleStatus> {
        let puzzle = self.puzzles.get(&solution_hash);
        puzzle.map(|p| p.status)
    }

    pub fn get_solution(&self, puzzle_index: u32) -> Option<String> {
        let mut index = 0;
        for puzzle_hash in self.unsolved_puzzles.iter() {
            if puzzle_index == index {
                return Some(puzzle_hash);
            }
            index += 1;
        }
        // Did not find that index
        None
    }

    pub fn get_unsolved_puzzles(&self) -> UnsolvedPuzzles {
        let solution_hashes = self.unsolved_puzzles.to_vec();
        let mut all_unsolved_puzzles = vec![];
        for hash in solution_hashes {
            let puzzle = self
                .puzzles
                .get(&hash)
                .unwrap_or_else(|| env::panic_str("ERR_LOADING_PUZZLE"));
            let json_puzzle = JsonPuzzle {
                solution_hash: hash,
                status: puzzle.status,
                answer: puzzle.answer,
            };
            all_unsolved_puzzles.push(json_puzzle)
        }
        UnsolvedPuzzles {
            puzzles: all_unsolved_puzzles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    fn get_answers() -> Vec<Answer> {
        vec![
            Answer {
                num: 1,
                start: CoordinatePair { x: 2, y: 1 },
                direction: AnswerDirection::Across,
                length: 4,
                clue: "Native token".to_string(),
            },
            Answer {
                num: 1,
                start: CoordinatePair { x: 2, y: 1 },
                direction: AnswerDirection::Down,
                length: 7,
                clue: "Name of the specs/standards site is ______.io".to_string(),
            },
            Answer {
                num: 2,
                start: CoordinatePair { x: 5, y: 1 },
                direction: AnswerDirection::Down,
                length: 3,
                clue: "DeFi site on NEAR is ___.finance".to_string(),
            },
            Answer {
                num: 4,
                start: CoordinatePair { x: 0, y: 7 },
                direction: AnswerDirection::Across,
                length: 7,
                clue: "DeFi decentralizes this".to_string(),
            },
        ]
    }

    #[test]
    fn debug_get_hash() {
        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "near nomicon ref finance";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Let's debug: {:?}", debug_hash_string);
    }

    #[test]
    #[should_panic(expected = "ERR_NOT_CORRECT_ANSWER")]
    fn check_submit_solution_failure() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        let context = get_context(alice.clone());
        testing_env!(context.build());

        let mut contract = Crossword::new(alice);

        let answers = get_answers();
        let solution_hash = "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f";
        contract.new_puzzle(solution_hash.to_string(), answers);
        contract.submit_solution("wrong answer here".to_string(), "my memo".to_string());
    }

    #[test]
    fn check_submit_solution_success() {
        let alice = AccountId::new_unchecked("alice.testnet".to_string());
        let context = get_context(alice.clone());
        testing_env!(context.build());

        let mut contract = Crossword::new(alice);

        let solution_hash = "69c2feb084439956193f4c21936025f14a5a5a78979d67ae34762e18a7206a0f";
        let answers = get_answers();
        contract.new_puzzle(solution_hash.to_string(), answers);
        contract.submit_solution(
            "near nomicon ref finance".to_string(),
            "my memo".to_string(),
        );

        let status = contract.get_puzzle_status(solution_hash.to_string());
        assert_eq!(
            status,
            Some(PuzzleStatus::Solved {
                memo: "my memo".to_string()
            })
        );
        assert_eq!(
            contract.unsolved_puzzles.len(),
            0,
            "Should not have any unsolved puzzles."
        );
    }
}
