use crate::{
    layouts::{authenticated::Authenticated, tabs::Tabs, user::User},
    pages::{
        account::Account, bill::Bill, dashboard::Dashboard, deposit::Deposit, explore::Explore,
        initiative::Initiative, initiatives::Initiatives, invoice::Invoice, login::Login,
        not_found::PageNotFound, onboarding::Onboarding, plugins::Plugins, send::Send,
        treasury::Treasury, vote::Vote, withdraw::Withdraw,
    },
};
use dioxus::prelude::*;
#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/onboarding")]
    Onboarding {},
    #[route("/login")]
    Login {},
    #[layout(Authenticated)]
        #[route("/account")]
        Account {},
        #[route("/withdraw")]
        Withdraw {},
        #[route("/deposit")]
        Deposit {},
        #[layout(User)]
            #[route("/")]
            Dashboard {},
            #[route("/explore")]
            Explore {},
        #[end_layout]
        #[layout(Tabs)]
            #[nest("/dao")]
                #[nest("/:id")]
                    #[route("/plugins")]
                    Plugins {id: u16},
                    #[route("/initiatives")]
                    Initiatives {id: u16},
                    #[route("/initiative")]
                    Initiative {id: u16},
                    #[route("/treasury")]
                    Treasury {id: u16},
                    #[route("/send")]
                    Send {id: u16},
                    #[route("/bill")]
                    Bill {id: u16},
                    #[route("/invoice")]
                    Invoice {id: u16},
                    #[route("/vote/:initiativeid")]
                    Vote {id: u16, initiativeid: u16},
                    #[end_nest]
                #[end_nest]
        #[end_layout]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
