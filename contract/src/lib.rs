use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    val: u8,
}

#[near_bindgen]
impl Counter {
    pub fn get_num(&self) -> u8 {
        self.val
    }

    pub fn increment(&mut self) {
        match self.val.checked_add(1) {
            Some(n) => {
                self.val = n;
                let log_message = format!("Increased counter to {}", self.val);
                env::log(log_message.as_bytes());
            }
            None => {
                env::panic(b"increment will lead to overflow");
            }
        }
    }

    pub fn decrement(&mut self) {
        match self.val.checked_sub(1) {
            Some(n) => {
                self.val = n;
                let log_message = format!("Decreased counter to {}", self.val);
                env::log(log_message.as_bytes());
            }
            None => {
                env::panic(b"decrement will lead to underflow");
            }
        }
    }

    pub fn reset(&mut self) {
        if env::predecessor_account_id() != env::current_account_id() {
            env::panic( b"Only the contract owner can call the reset method")
        }
        self.val = 0;
        env::log(b"Reset counter to zero");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{MockedBlockchain};
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn increment() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Counter::default();
        contract.increment();
        println!("Value after increment: {}", contract.get_num());

        assert_eq!(1, contract.get_num());
    }

    #[test]
    fn decrement() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Counter { val: 2 };
        contract.decrement();
        println!("Value after decrement: {}", contract.get_num());

        assert_eq!(1, contract.get_num());
    }

    #[test]
    #[should_panic(expected = "increment will lead to overflow")]
    fn increment_overflow() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Counter { val: u8::MAX };
        contract.increment();
    }

    #[test]
    #[should_panic(expected = "decrement will lead to underflow")]
    fn decrement_underflow() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Counter { val: 0 };
        contract.decrement();
    }

    #[test]
    fn increment_and_reset() {
        let mut context = get_context(vec![], false);
        context.predecessor_account_id = context.current_account_id.clone();
        testing_env!(context);

        let mut contract = Counter { val: 0 };
        contract.increment();
        assert_eq!(1, contract.get_num());
        contract.reset();
        println!("Value after reset {}", contract.get_num());
        assert_eq!(0, contract.get_num());
    }
}
