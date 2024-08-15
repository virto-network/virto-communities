#![allow(non_snake_case)]
pub mod pages {
    pub mod dashboard;
    pub mod explore;
    pub mod initiative;
    pub mod initiatives;
    pub mod not_found;
    pub mod onboarding;
    pub mod route;
    pub mod vote;
}

pub mod middlewares {
    pub mod is_chain_available;
    pub mod is_dao_owner;
}

pub mod layouts {
    pub mod authenticated;
    pub mod onboard;
}

pub mod hooks {
    pub mod use_accounts;
    pub mod use_attach;
    pub mod use_communities;
    pub mod use_connect_wallet;
    pub mod use_initiative;
    pub mod use_language;
    pub mod use_notification;
    pub mod use_onboard;
    pub mod use_our_navigator;
    pub mod use_paginator;
    pub mod use_session;
    pub mod use_spaces_client;
    pub mod use_startup;
    pub mod use_theme;
    pub mod use_timestamp;
    pub mod use_tooltip;
    pub mod use_vote;
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
        pub mod system;
    }
}
