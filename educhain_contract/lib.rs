#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod errors;
pub mod events;

#[ink::contract]
mod educhain_contract {
    type Err = crate::errors::Errores;

    use ink::storage::Mapping;

    #[derive(PartialEq, Eq)]
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
            Self {
                roles,
                students_certificates,
            }
        }
        #[ink(message)]
        pub fn grant_role(&mut self, address: Address, role: Role) -> Result<(), Err> {
            self._grant_role(address, role)?;
            Ok(())
        }
        fn _grant_role(&mut self, address: Address, role: Role) -> Result<(), Err> {
            if let Some(address_role) = self.roles.get(address) {
                if address_role == role {
                    return Err(Err::SameRole);
                }
            }
            self.roles.insert(address, &role);
            Ok(())
        }
        #[ink(message)]
        pub fn revoke_role(&mut self, address: Address) -> Result<(), Err> {
            Ok(())
        }
    }
}
