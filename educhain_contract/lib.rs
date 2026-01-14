#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod events;
pub mod errors;

#[ink::contract]
mod educhain_contract {
    use ink::storage::Mapping;
    use crate::events::AdminAddedEvent;
    use ink::env::ReturnFlags as revert;

    #[ink::storage_item(packed)]
    pub enum Role {
        Owner,
        Admin,
        Public,
    }

    #[ink::storage_item(packed)]
    pub enum CertificateState {
        Active,
        Revoked,
    }

    #[ink::storage_item(packed)]
    pub struct Certificate {
        id: u64,
        studen_address: Address,
        course_name: String,
        issue_date: Timestamp,
        state: CertificateState,
        certificate_hash: Hash,
    }

    // pub struct

    #[ink(storage)]
    pub struct Educhain {
        roles: Mapping<Address, Role>,
        students_certificates: Mapping<Address, Certificate>,
    }
    impl Educhain {
        #[ink(constructor)]
        pub fn new() -> Self {
            let owner = Self::env().caller();
            let mut roles = Mapping::new();
            roles.insert(owner, &Role::Owner);
            let students_certificates = Mapping::new();
            Self::env().emit_event(AdminAddedEvent {
                admin_added: Some(owner),
                owner: Some(owner),
            });
            Self {
                roles,
                students_certificates,
            }
        }
        #[ink(message)]
        pub fn add_admin(&mut self, admin_added: Address) {
            
            if let Some(role) =  self.roles.get(admin_added) {
                
            }
            
        }   
    }
}
