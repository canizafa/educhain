#[derive(PartialEq, Eq, Debug)]
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

