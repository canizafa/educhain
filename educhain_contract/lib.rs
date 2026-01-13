#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod educhain_contract {
    use ink::storage::Mapping;

    #[ink::storage_item(packed)]
    pub enum Role {
        Owner,
        Admin,
    }

    #[ink::storage_item(packed)]
    pub enum CertificateState {
        Active,
        Revoked,
    }

    pub struct Certificate {
        id: u64,
        studen_address: Address,
        course_name: String,
        issue_date: Timestamp,
        state: CertificateState,
    }

    #[ink(storage)]
    pub struct Educhain {
        roles: Mapping<Address, Role>,
        

    }
    impl Educhain {
        #[ink(constructor)]
        pub fn new() -> Self {
            let owner = Self::env().caller();
            let mut mapping = Mapping::new();
            mapping.insert(owner, &Role::Owner);
            Self { roles: mapping }
        }
        #[ink(message)]
        pub fn hello(&self) {}
    }
}
