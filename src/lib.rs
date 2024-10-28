#![allow(non_snake_case)]
pub mod pages {
    pub mod account;
    pub mod bill;
    pub mod dashboard;
    pub mod deposit;
    pub mod explore;
    pub mod initiative;
    pub mod initiatives;
    pub mod invoice;
    pub mod login;
    pub mod not_found;
    pub mod onboarding;
    pub mod plugins;
    pub mod route;
    pub mod send;
    pub mod treasury;
    pub mod vote;
    pub mod withdraw;
}
pub mod middlewares {
    pub mod is_chain_available;
    pub mod is_dao_owner;
}
pub mod layouts {
    pub mod authenticated;
    pub mod tabs;
    pub mod user;
}
pub mod hooks {
    pub mod use_accounts;
    pub mod use_attach;
    pub mod use_bill;
    pub mod use_communities;
    pub mod use_connect_wallet;
    pub mod use_deposit;
    pub mod use_initiative;
    pub mod use_invoice;
    pub mod use_language;
    pub mod use_location;
    pub mod use_market_client;
    pub mod use_notification;
    pub mod use_onboard;
    pub mod use_our_navigator;
    pub mod use_paginator;
    pub mod use_recipient;
    pub mod use_recipients;
    pub mod use_send;
    pub mod use_session;
    pub mod use_paginator;
    pub mod use_spaces_client;
    pub mod use_startup;
    pub mod use_tabs;
    pub mod use_theme;
    pub mod use_timestamp;
    pub mod use_tooltip;
    pub mod use_vote;
    pub mod use_withdraw;
}
pub mod components {
    pub mod atoms;
    pub mod molecules;
}
pub mod services {
    pub mod bot {
        pub mod client;
        pub mod types;
    }
    pub mod market {
        pub mod client;
        pub mod types;
    }
    pub mod kreivo {
        pub mod balances;
        pub mod communities;
        pub mod community_memberships;
        pub mod community_referenda;
        pub mod community_track;
        pub mod identity;
        pub mod preimage;
        pub mod system;
        pub mod timestamp;
    }
    pub mod kusama {
        pub mod balances;
        pub mod system;
    }
}
