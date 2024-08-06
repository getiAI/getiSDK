#![no_std]
use stylus_sdk::{sol_storage, msg, contract, entrypoint};
use alloc::vec::Vec;

sol_storage! {
    pub struct SimpleStorage {
        uint256 value;
    }
}

impl SimpleStorage {
    pub fn set_value(&mut self, value: u64) {
        self.value.set(value.into());
    }

    pub fn get_value(&self) -> u64 {
        self.value.get().as_u64()
    }
}

#[entrypoint]
fn user_main(input: Vec<u8>) -> Vec<u8> {
    let mut storage = contract::Storage::<SimpleStorage>::load();
    if input.is_empty() {
        storage.get_value().to_le_bytes().to_vec()
    } else {
        let value = u64::from_le_bytes(input.try_into().unwrap());
        storage.set_value(value);
        Vec::new()
    }
}
