use crate::educhain_contract::{CertificateState, Role};
use ink::{Address, env::Timestamp};

#[ink::event]
pub struct RoleGranted {
    #[ink(topic)]
    pub address_granted: Address,
    pub role_granted: Role,
}

#[ink::event]
pub struct RoleRevoked {
    #[ink(topic)]
    pub address_revoked: Address,
    pub role_revoked: Role,
}

#[ink::event]
pub struct OwnerTransferred {
    #[ink(topic)]
    pub address_old_owner: Address,
    #[ink(topic)]
    pub address_new_owner: Address,
}

#[ink::event]
pub struct EmitCertificateEvent {
    #[ink(topic)]
    pub student_address: Address,
    pub course_name: String,
    pub certificate_state: CertificateState,
    #[ink(topic)]
    pub issue_date: Timestamp,
}

#[ink::event]
pub struct CertificationRevokedEvent {
    #[ink(topic)]
    pub student_address: Address,
    #[ink(topic)]
    pub certificate_id: u64
}