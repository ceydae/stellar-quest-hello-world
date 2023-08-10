#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Symbol};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello_world() -> Symbol {
        symbol_short!("Hello")
    }
}

#[cfg(test)]
mod test;