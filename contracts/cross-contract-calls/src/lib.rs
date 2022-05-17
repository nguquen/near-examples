// https://docs.near.org/docs/tutorials/contracts/xcc-receipts
// https://nomicon.io/RuntimeSpec/Scenarios/CrossContractCall

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, ext_contract, near_bindgen, promise_result_as_success, AccountId, Promise,
};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[ext_contract(ext_counter_contract)]
trait CounterContract {
    fn get_num(&self) -> i8;

    fn increment(&self);
}

#[ext_contract(ext_self)]
pub trait ContractResolver {
    fn resolve_increment(&self) -> i8;
}

#[near_bindgen]
impl Contract {
    pub fn check_counter(&self, ext_contract_id: AccountId) -> Promise {
        ext_counter_contract::get_num(&ext_contract_id, 0, 5_000_000_000_000)
    }

    pub fn increment_counter(&self, ext_contract_id: AccountId) -> Promise {
        ext_counter_contract::increment(&ext_contract_id, 0, 5_000_000_000_000).then(
            ext_self::resolve_increment(&env::current_account_id(), 0, 5_000_000_000_000),
        )
    }

    #[private]
    pub fn resolve_increment(&self) -> i8 {
        if let Some(val) = promise_result_as_success() {
            let result = near_sdk::serde_json::from_slice::<i8>(&val).unwrap();
            env::log(format!("The counter.increment result is: {}", result).as_bytes());
            return result;
        } else {
            env::panic(b"counter.increment failed");
        }
    }
}
