// https://docs.near.org/docs/tutorials/contracts/xcc-receipts
// https://nomicon.io/RuntimeSpec/Scenarios/CrossContractCall

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    ext_contract, near_bindgen, AccountId, Promise,
};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {}

#[ext_contract(ext_counter_contract)]
trait CounterContract {
    fn get_num(&self) -> i8;

    fn increment(&self);
}

#[near_bindgen]
impl Contract {
    pub fn check_counter(&self, ext_contract_id: AccountId) -> Promise {
        ext_counter_contract::get_num(&ext_contract_id, 0, 5_000_000_000_000)
    }

    pub fn increment_counter(&self, ext_contract_id: AccountId) {
        ext_counter_contract::increment(&ext_contract_id, 0, 5_000_000_000_000);
    }
}
