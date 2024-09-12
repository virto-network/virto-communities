use crate::{
    layouts::{authenticated::Authenticated, onboard::Onboard},
    pages::{
        account::Account, dashboard::Dashboard, explore::Explore, initiative::Initiative,
        initiatives::Initiatives, login::Login, not_found::PageNotFound, onboarding::Onboarding,
        payment::Payment, vote::Vote, withdraw::Withdraw,
    },
};
use dioxus::prelude::*;
#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/onboarding")]
    Onboarding {},
    #[route("/payment")]
    Payment {},
    #[route("/login")]
    Login {},
    #[layout(Authenticated)]
        #[route("/")]
        Dashboard {},
        #[route("/account")]
        Account {},
        #[route("/withdraw")]
        Withdraw {},
        #[layout(Onboard)]
            #[route("/explore")]
            Explore {},
            #[nest("/dao")]
                #[nest("/:id")]
                    #[route("/initiatives")]
                    Initiatives {id: u16},
                    #[route("/initiative")]
                    Initiative {id: u16},
                    #[route("/vote/:initiativeid")]
                    Vote {id: u16, initiativeid: u16},
                    #[end_nest]
                #[end_nest]
        #[end_layout]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
