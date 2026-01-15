#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub mod errors;
pub mod events;

#[ink::contract]
mod educhain_contract {
    type Err = crate::errors::Errors;

    use crate::events;
    use ink::prelude::{string::String, vec::Vec};
    use ink::storage::Mapping;

    const ZERO_ADDRESS: Address = Address::zero();

    #[derive(PartialEq, Eq)]
    #[ink::storage_item(packed)]
    pub enum Role {
        Admin,
        Public,
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    #[ink::storage_item(packed)]
    pub enum CertificateState {
        Active,
        Revoked,
    }

    #[derive(Clone)]
    #[ink::storage_item(packed)]
    pub struct Certificate {
        id: u64,
        student_address: Address,
        course_name: String,
        issue_date: Timestamp,
        certificate_state: CertificateState,
        certificate_hash: Hash,
    }

    #[ink(storage)]
    pub struct Educhain {
        owner: Address,
        roles: Mapping<Address, Role>,
        students_certificates: Mapping<Address, Vec<Certificate>>,
        certificates: Mapping<u64, Certificate>,
        next_id: u64,
    }
    impl Educhain {
        #[ink(constructor)]
        pub fn new() -> Self {
            let owner = Self::env().caller();
            let mut roles = Mapping::new();
            roles.insert(owner, &Role::Admin);
            let students_certificates = Mapping::new();
            let certificates = Mapping::new();
            Self {
                owner,
                roles,
                students_certificates,
                certificates,
                next_id: 0,
            }
        }
        fn invalid_address(&self, address: Address) -> Result<(), Err> {
            if address == ZERO_ADDRESS {
                return Err(Err::ZeroAddress);
            }
            Ok(())
        }
        fn owner_or_admin(&self, address: Address) -> Result<(), Err> {
            if let Some(address_role) = self.roles.get(address) {
                if address_role == Role::Public {
                    return Err(Err::NonAuthorizaded);
                }
                return Ok(());
            }
            return Err(Err::InvalidOwner);
        }
        #[ink(message)]
        pub fn grant_role(&mut self, address: Address, role: Role) -> Result<(), Err> {
            self._grant_role(address, &role)?;
            self.invalid_address(address)?;
            self.env().emit_event(events::RoleGranted {
                address_granted: address,
                role_granted: role,
            });
            Ok(())
        }
        fn _grant_role(&mut self, address: Address, role: &Role) -> Result<(), Err> {
            if let Some(address_role) = self.roles.get(address) {
                if address_role == *role {
                    return Err(Err::SameRole);
                }
            }
            self.roles.insert(address, role);
            Ok(())
        }
        #[ink(message)]
        pub fn revoke_role(&mut self, address: Address) -> Result<(), Err> {
            self.invalid_address(address)?;
            let role_revoked = self._revoke_role(address)?;
            self.env().emit_event(events::RoleRevoked {
                address_revoked: address,
                role_revoked: role_revoked,
            });
            Ok(())
        }
        fn _revoke_role(&mut self, address: Address) -> Result<Role, Err> {
            if let Some(address_role) = self.roles.get(address) {
                if address_role == Role::Public {
                    return Err(Err::CantRevokePublicRole);
                }
                self.roles.insert(address, &Role::Public);
                return Ok(address_role);
            }
            Err(Err::NotAddressFound)
        }
        #[ink(message)]
        pub fn transfer_owner(&mut self, new_owner: Address) -> Result<(), Err> {
            self.invalid_address(new_owner)?;
            self._transfer_owner(new_owner)?;
            self.env().emit_event(events::OwnerTransferred {
                address_old_owner: self.owner,
                address_new_owner: new_owner,
            });
            Ok(())
        }
        fn _transfer_owner(&mut self, new_owner: Address) -> Result<(), Err> {
            if new_owner == self.owner {
                return Err(Err::InvalidOwner);
            }
            self.owner = new_owner;
            Ok(())
        }
        #[ink(message)]
        pub fn emit_certification(
            &mut self,
            student_address: Address,
            course_name: String,
            certificate_state: CertificateState,
            certificate_hash: Hash,
        ) -> Result<(), Err> {
            let caller = self.env().caller();
            self.owner_or_admin(caller)?;
            self.invalid_address(student_address)?;
            let issue_date = self.env().block_timestamp();
            self._emit_certification(
                student_address,
                course_name.clone(),
                certificate_state.clone(),
                certificate_hash,
                issue_date,
            );
            self.env().emit_event(events::EmitCertificateEvent {
                student_address,
                course_name,
                certificate_state,
                issue_date,
            });
            Ok(())
        }
        fn _emit_certification(
            &mut self,
            student_address: Address,
            course_name: String,
            certificate_state: CertificateState,
            certificate_hash: Hash,
            issue_date: Timestamp,
        ) {
            let certificate = Certificate {
                id: self.next_id,
                student_address,
                course_name,
                issue_date,
                certificate_state,
                certificate_hash,
            };
            if let Some(mut collection) = self.students_certificates.get(student_address) {
                collection.push(certificate.clone());
                self.students_certificates
                    .insert(student_address, &collection);
            }
            self.certificates.insert(self.next_id, &certificate);
            self.next_id += 1;
        }
        #[ink(message)]
        pub fn get_certificate(&self, id: u64) -> Result<Certificate, Err> {
            if let Some(certificate) = self.certificates.get(id) {
                Ok(certificate)
            } else {
                Err(Err::InvalidID)
            }
        }
        #[ink(message)]
        pub fn is_valid_certificate(&self, id: u64) -> bool {
            if let Some(certificate) = self.certificates.get(id) {
                if certificate.certificate_state != CertificateState::Active {
                    false
                } else {
                    true
                }
            } else {
                false
            }
        }
        #[ink(message)]
        pub fn revoke_certificate(&mut self, id: u64) -> Result<(), Err> {
            let caller = self.env().caller();
            self.owner_or_admin(caller)?;
            let student_address = self._revoke_certification(id)?;
            self.env().emit_event(events::CertificationRevokedEvent {
                student_address,
                certificate_id: id,
            });
            Ok(())
        }
        fn _revoke_certification(&mut self, id: u64) -> Result<Address, Err> {
            if let Some(mut certificate) = self.certificates.get(id) {
                let student_address = certificate.student_address;
                if let Some(mut collection) = self.students_certificates.get(student_address) {
                    if let Some(cert) = collection.iter_mut().find(|c| c.id == id) {
                        cert.certificate_state = CertificateState::Revoked;
                    }
                    self.students_certificates
                        .insert(student_address, &collection);
                }
                certificate.certificate_state = CertificateState::Revoked;
                self.certificates.insert(id, &certificate);
                return Ok(student_address);
            }
            return Err(Err::InvalidID);
        }
        #[ink(message)]
        pub fn get_certificates_of_student(
            &self,
            address: Address,
        ) -> Result<Vec<Certificate>, Err> {
            self.invalid_address(address)?;
            Ok(self._get_certificates_of_student(address))
        }
        fn _get_certificates_of_student(&self, address: Address) -> Vec<Certificate> {
            match self.students_certificates.get(address) {
                Some(collection) => collection,
                None => Vec::new(),
            }
        }
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        fn create_app() -> Educhain {
            Educhain::new()
        }

        #[ink::test]
        fn test_creation() {
            let app = create_app();
            assert_eq!(app.owner, Address::zero());
        }
        #[ink::test]
        fn invalid_address() {
            let app = create_app();
            let result = app.invalid_address(Address::zero());
            assert_eq!(result, Err(Err::ZeroAddress));
        }
        #[ink::test]
        fn owner_invalid() {
            let mut app = create_app();
            app.owner = Address::zero();
            let result = app.owner_or_admin(Address::random());
            assert_eq!(result, Err(Err::InvalidOwner));
        }
    }
}
