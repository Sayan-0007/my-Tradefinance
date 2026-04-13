#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, Address, Symbol,
};

#[contract]
pub struct TradeFinanceContract;

#[contracttype]
#[derive(Clone)]
pub struct Trade {
    pub buyer: Address,
    pub seller: Address,
    pub amount: i128,
    pub status: Symbol,
}

// Storage key
#[contracttype]
pub enum DataKey {
    Trade,
}

#[contractimpl]
impl TradeFinanceContract {

    // Initialize trade
    pub fn init_trade(env: Env, buyer: Address, seller: Address, amount: i128) {
        buyer.require_auth();

        let trade = Trade {
            buyer: buyer.clone(),
            seller,
            amount,
            status: symbol_short!("INIT"),
        };

        env.storage().instance().set(&DataKey::Trade, &trade);
    }

    // Deposit (mock escrow)
    pub fn deposit(env: Env, buyer: Address) {
        buyer.require_auth();

        let mut trade: Trade = env.storage()
            .instance()
            .get(&DataKey::Trade)
            .unwrap();

        if trade.status != symbol_short!("INIT") {
            panic!("Invalid state");
        }

        trade.status = symbol_short!("FUNDED");

        env.storage().instance().set(&DataKey::Trade, &trade);
    }

    // Seller confirms shipment
    pub fn confirm_shipment(env: Env, seller: Address) {
        seller.require_auth();

        let mut trade: Trade = env.storage()
            .instance()
            .get(&DataKey::Trade)
            .unwrap();

        if trade.status != symbol_short!("FUNDED") {
            panic!("Funds not deposited");
        }

        trade.status = symbol_short!("SHIPPED");

        env.storage().instance().set(&DataKey::Trade, &trade);
    }

    // Buyer confirms delivery
    pub fn confirm_delivery(env: Env, buyer: Address) {
        buyer.require_auth();

        let mut trade: Trade = env.storage()
            .instance()
            .get(&DataKey::Trade)
            .unwrap();

        if trade.status != symbol_short!("SHIPPED") {
            panic!("Goods not shipped");
        }

        trade.status = symbol_short!("COMPLETED");

        env.storage().instance().set(&DataKey::Trade, &trade);
    }

    // Get trade details
    pub fn get_trade(env: Env) -> Trade {
        env.storage()
            .instance()
            .get(&DataKey::Trade)
            .unwrap()
    }
}