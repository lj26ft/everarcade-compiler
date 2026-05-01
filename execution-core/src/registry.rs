use crate::{
    contract::Contract,
    contracts::{
        increment::IncrementContract,
        set::SetContract,
    },
};

pub struct ContractRegistry;

impl ContractRegistry {
    pub fn execute(
        contract_id: &str,
    ) -> Box<dyn Contract> {
        match contract_id {
            "set" => Box::new(SetContract),
            "increment" => Box::new(IncrementContract),
            _ => panic!("unknown contract"),
        }
    }
}
