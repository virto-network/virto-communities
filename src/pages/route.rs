use dioxus::prelude::*;

use crate::{
    layouts::{authenticated::Authenticated, onboard::Onboard},
    pages::{dashboard::Dashboard, not_found::PageNotFound},
};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Authenticated)]
        #[route("/")]
        Dashboard {},
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
