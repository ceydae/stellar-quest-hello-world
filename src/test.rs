use crate::{Contract, ContractClient};
use soroban_sdk::{Env, symbol_short};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Contract);
    let contract = ContractClient::new(&env, &contract_id);

    assert_eq!(contract.hello_world(), symbol_short!("Hello"));
}
