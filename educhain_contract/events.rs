use ink::Address;

    #[ink::event]
    pub struct AdminAddedEvent {
        #[ink(topic)]
        pub admin_added: Option<Address>,
        #[ink(topic)]
        pub owner: Option<Address>,
    }