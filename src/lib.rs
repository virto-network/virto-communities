#![allow(non_snake_case)]
pub mod pages {
    pub mod dashboard;
    pub mod not_found;
    pub mod onboarding;
    pub mod route;
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
    pub mod use_language;
    pub mod use_notification;
    pub mod use_onboard;
    pub mod use_paginator;
    pub mod use_session;
    pub mod use_startup;
    pub mod use_theme;
    pub mod use_tooltip;
}

pub mod components {
    pub mod atoms;
    pub mod molecules;
}

pub mod services {
    pub mod bot {
        pub mod create;
        pub mod get_by_id;
        pub mod upload;
    }
    pub mod kreivo {
        pub mod balances;
        pub mod community_memberships;
        pub mod community_track;
    }
}
