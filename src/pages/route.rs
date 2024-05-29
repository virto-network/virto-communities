use dioxus::prelude::*;

use crate::{
    layouts::{authenticated::Authenticated, onboard::Onboard},
    pages::{dashboard::Dashboard, initiatives::Initiatives, not_found::PageNotFound},
};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Authenticated)]
        #[route("/")]
        Dashboard {},
        #[layout(Onboard)]
            #[route("/initiatives")]
            Initiatives {},
        #[end_layout]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
