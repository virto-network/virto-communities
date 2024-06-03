use dioxus::prelude::*;

use crate::{
    layouts::{authenticated::Authenticated, onboard::Onboard},
    pages::{dashboard::Dashboard, not_found::PageNotFound, onboarding::Onboarding},
};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/onboarding")]
    Onboarding {},
    #[layout(Authenticated)]
        #[route("/")]
        Dashboard {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
