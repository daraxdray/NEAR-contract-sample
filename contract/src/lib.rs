use near_sdk::{near_bindgen, env, Gas, AccountId};
use near_sdk::borsh::{self,BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::store::LookupMap;
// setup_alloc!();

#[near_bindgen]
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Counter{
    value: u32,
    names: Vector<String>
}
pub struct Counter2{
    count: LookupMap<String, u64>,
    name: Vector<String>
}



impl Default for Counter {
    fn default()->  Self {
        Self{ value: 0 , names: Vector::new(b"n")}
    }
}
#[near_bindgen]
impl Counter{

    pub fn set_name(&mut self, name: String) -> bool {
        if name.is_empty() {
            self.names.push(&name);
            true
        }else {
            false
        }
    }

    pub fn get_name(&self) -> Vec<String> {
        let mut result : Vec<String> = vec![];
        for name in self.names.iter(){
            result.push(name);
        }
        result
    }
    pub fn remove_name(&mut self, index : u32) -> String {
            self.names.remove(index)
    }
    pub fn read_value(&self) -> u32{
        self.value
    }

    pub fn decrement(&mut self)-> u32{
        if self.value > 0 {
            self.value -= 1;
        }
        self.value
    }



    pub fn increment(&mut self) -> u32{
        self.value += 1;
        self.value
    }

    pub fn wallet_address(&self) -> String{
        String::from(env::signer_account_id())
    }

    #[payable]
    pub fn attached_near(&mut self) -> u128{
        env::attached_deposit()
    }

    pub fn gas_used(&self) -> (Gas,Gas){
        let attached_gas:Gas = env::prepaid_gas();
        let gas_used: Gas = env::used_gas();
        (attached_gas, gas_used)
    }


}

impl Counter2{
    pub fn read_value_2(&self) -> u64{
        let signer: AccountId = env::signer_account_id();
        match self.count.get(&signer.to_string()) {
            Some(counter) => counter,
            None => 0
        }
    }

    pub fn decrement_2(&mut self)-> u64{
        let signer : AccountId = env::signer_account_id();
        if let Some(mut counter) = self.count.get(&signer.to_string()){
            if counter > 0 {
                counter -= 1;
                self.count.insert(&signer.to_string(),&counter);
            }
            counter
        }else{
            0
        }


    }

    pub fn increment_2(&mut self)-> u64{
        let signer : AccountId = env::signer_account_id();
        if let Some(mut counter) = self.count.get(&signer.to_string()){
            counter += 1;
            self.count.insert(&signer.to_string(),&counter);
            counter
        }else{
            0
        }


    }
}