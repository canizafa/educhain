use crate::educhain_contract::Role;
use ink::Address;

#[ink::event]
pub struct RoleGranted {
    #[ink(topic)]
    address_granted: Address,
    role_granted: Role,
}
