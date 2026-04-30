#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, Symbol,
};

#[contracttype]
#[derive(Clone)]
pub struct Customer {
    pub debt: i128,
}

const CUSTOMER: Symbol = symbol_short!("CUSTOM");

#[contract]
pub struct SariSaveContract;

#[contractimpl]
impl SariSaveContract {

    // Create customer
    pub fn create_customer(env: Env, customer: Address) {
        customer.require_auth();

        let data = Customer {
            debt: 0,
        };

        env.storage()
            .persistent()
            .set(&(CUSTOMER, customer), &data);
    }

    // Add utang
    pub fn issue_credit(env: Env, customer: Address, amount: i128) {

        let mut data: Customer = env
            .storage()
            .persistent()
            .get(&(CUSTOMER, customer.clone()))
            .unwrap();

        data.debt += amount;

        env.storage()
            .persistent()
            .set(&(CUSTOMER, customer), &data);
    }

    // Repay utang
    pub fn repay_credit(env: Env, customer: Address, amount: i128) {

        let mut data: Customer = env
            .storage()
            .persistent()
            .get(&(CUSTOMER, customer.clone()))
            .unwrap();

        data.debt -= amount;

        if data.debt < 0 {
            data.debt = 0;
        }

        env.storage()
            .persistent()
            .set(&(CUSTOMER, customer), &data);
    }

    // View remaining utang
    pub fn view_balance(env: Env, customer: Address) -> i128 {

        let data: Customer = env
            .storage()
            .persistent()
            .get(&(CUSTOMER, customer))
            .unwrap();

        data.debt
    }
}