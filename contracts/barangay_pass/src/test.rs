#![cfg(test)]

mod tests {
    use soroban_sdk::{
        testutils::Address as _,
        Address, Env,
    };

    use crate::{BarangayPass, BarangayPassClient};

    fn setup() -> (Env, BarangayPassClient<'static>, Address, Address) {
        let env = Env::default();

        let contract_id = env.register(BarangayPass, ());
        let client = BarangayPassClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let resident = Address::generate(&env);

        env.mock_all_auths();

        client.init(&admin);

        (env, client, admin, resident)
    }

    /// Test 1 — Happy path
    #[test]
    fn resident_registers_successfully() {
        let (_env, client, _admin, resident) = setup();

        client.create_event(&1u32, &50i128);
        client.register_resident(&1u32, &resident, &50i128);

        assert!(client.verify_registration(&1u32, &resident));
    }

    /// Test 2 — Duplicate registration rejected
    #[test]
    #[should_panic(expected = "already registered")]
    fn duplicate_registration_fails() {
        let (_env, client, _admin, resident) = setup();

        client.create_event(&2u32, &100i128);

        client.register_resident(&2u32, &resident, &100i128);
        client.register_resident(&2u32, &resident, &100i128);
    }

    /// Test 3 — State verification
    #[test]
    fn registration_state_is_correct() {
        let (_env, client, _admin, resident) = setup();

        client.create_event(&3u32, &75i128);
        client.register_resident(&3u32, &resident, &75i128);

        let verified = client.verify_registration(&3u32, &resident);

        assert_eq!(verified, true);
    }
}
