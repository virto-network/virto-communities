use dioxus::prelude::*;

use crate::{
    layouts::{authenticated::Authenticated, onboard::Onboard},
    pages::{
        dashboard::Dashboard, explore::Explore, initiatives::Initiatives, not_found::PageNotFound,
        onboarding::Onboarding,
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
            #[route("/dao/:id")]
            Initiatives {id: u16},
        #[end_layout]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
