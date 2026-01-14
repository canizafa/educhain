#[ink::error]
pub enum Errors {
    InvalidOwner,
    SameRole,
    CantRevokePublicRole,
    NotAddressFound,
    ZeroAddress,
    NonAuthorizaded,
    InvalidID,
}

