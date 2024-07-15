use dioxus::prelude::*;

use crate::{
    layouts::{authenticated::Authenticated, onboard::Onboard},
    pages::{
        dashboard::Dashboard, explore::Explore, initiative::Initiative, initiatives::Initiatives,
        not_found::PageNotFound, onboarding::Onboarding, vote::Vote,
    },
};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/onboarding")]
    Onboarding {},
    #[layout(Authenticated)]
        #[route("/")]
        Dashboard {},
        #[layout(Onboard)]
            #[route("/explore")]
            Explore {},
            #[nest("/dao")]
                #[nest("/:id")]
                    #[route("/")]
                    Initiatives {id: u16},
                    #[end_nest]
                #[end_nest]
            #[route("/initiative")]
            Initiative {},
            #[route("/vote")]
            Vote {},
        #[end_layout]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
