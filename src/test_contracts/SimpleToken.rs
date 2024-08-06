#![no_std]
use stylus_sdk::{sol_storage, msg, contract, entrypoint};
use alloc::vec::Vec;
use alloc::string::String;

sol_storage! {
    pub struct SimpleToken {
        string name;
        string symbol;
        uint256 total_supply;
        mapping(address => uint256) balances;
    }
}

impl SimpleToken {
    pub fn new(name: String, symbol: String, total_supply: u64) -> Self {
        let mut token = SimpleToken {
            name: StorageString::new(name),
            symbol: StorageString::new(symbol),
            total_supply: StorageUint::new(total_supply.into()),
            balances: StorageMap::new(),
        };
        token.balances.insert(msg::sender(), total_supply.into());
        token
    }

    pub fn balance_of(&self, owner: address) -> u64 {
        self.balances.get(owner).unwrap_or_else(|| 0.into()).as_u64()
    }

    pub fn transfer(&mut self, to: address, amount: u64) -> bool {
        let sender_balance = self.balances.get(msg::sender()).unwrap_or_else(|| 0.into());
        if sender_balance < amount.into() {
            return false;
        }
        self.balances.insert(msg::sender(), sender_balance - amount.into());
        let receiver_balance = self.balances.get(to).unwrap_or_else(|| 0.into());
        self.balances.insert(to, receiver_balance + amount.into());
        true
    }
}

#[entrypoint]
fn user_main(input: Vec<u8>) -> Vec<u8> {
    // Entry point logic here
    Vec::new()
}
