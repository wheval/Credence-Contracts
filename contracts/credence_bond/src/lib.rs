#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone, Debug)]
pub struct IdentityBond {
    pub identity: Address,
    pub bonded_amount: i128,
    pub bond_start: u64,
    pub bond_duration: u64,
    pub slashed_amount: i128,
    pub active: bool,
}

#[contract]
pub struct CredenceBond;

#[contractimpl]
impl CredenceBond {
    /// Initialize the contract (admin).
    pub fn initialize(e: Env, admin: Address) {
        e.storage().instance().set(&Symbol::new(&e, "admin"), &admin);
    }

    /// Create or top-up a bond for an identity. In a full implementation this would
    /// transfer USDC from the caller and store the bond.
    pub fn create_bond(
        e: Env,
        identity: Address,
        amount: i128,
        duration: u64,
    ) -> IdentityBond {
        let bond = IdentityBond {
            identity: identity.clone(),
            bonded_amount: amount,
            bond_start: e.ledger().timestamp(),
            bond_duration: duration,
            slashed_amount: 0,
            active: true,
        };
        let key = Symbol::new(&e, "bond");
        e.storage().instance().set(&key, &bond);
        bond
    }

    /// Return current bond state for an identity (simplified: single bond per contract instance).
    pub fn get_identity_state(e: Env) -> IdentityBond {
        e.storage()
            .instance()
            .get::<_, IdentityBond>(&Symbol::new(&e, "bond"))
            .unwrap_or_else(|| {
                panic!("no bond")
            })
    }
}

#[cfg(test)]
mod test;
