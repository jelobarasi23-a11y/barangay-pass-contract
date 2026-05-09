#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};

#[contract]
pub struct BarangayPass;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    EventFee(u32),
    Registered(u32, Address),
}

#[contractimpl]
impl BarangayPass {
    /// Initialize contract with barangay admin wallet
    pub fn init(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Create a barangay event and define registration fee
    pub fn create_event(env: Env, event_id: u32, fee: i128) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .unwrap();

        admin.require_auth();

        if fee < 0 {
            panic!("invalid fee");
        }

        env.storage()
            .instance()
            .set(&DataKey::EventFee(event_id), &fee);
    }

    /// Register resident after payment is verified by frontend/backend
    pub fn register_resident(
        env: Env,
        event_id: u32,
        resident: Address,
        payment_amount: i128,
    ) {
        resident.require_auth();

        let fee: i128 = env
            .storage()
            .instance()
            .get(&DataKey::EventFee(event_id))
            .unwrap();

        if payment_amount < fee {
            panic!("insufficient payment");
        }

        let key = DataKey::Registered(event_id, resident.clone());

        if env.storage().instance().has(&key) {
            panic!("already registered");
        }

        env.storage().instance().set(&key, &true);

        // emit simple event
        env.events()
            .publish((symbol_short!("joined"), event_id), resident);
    }

    /// Verify if resident is registered
    pub fn verify_registration(env: Env, event_id: u32, resident: Address) -> bool {
        let key = DataKey::Registered(event_id, resident.clone());

        let verified: bool = env.storage().instance().get(&key).unwrap_or(false);

        env.events()
            .publish((symbol_short!("verify"), event_id), resident);

        verified
    }

    /// Return configured event fee
    pub fn get_event_fee(env: Env, event_id: u32) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::EventFee(event_id))
            .unwrap()
    }

    pub fn name(env: Env) -> Symbol {
        Symbol::new(&env, "BarangayPass")
    }
}

mod test;
