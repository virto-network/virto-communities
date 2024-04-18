use dioxus::prelude::*;

use crate::{
    layouts::authenticated::Authenticated, layouts::onboard::Onboard, pages::custom::Custom,
    pages::dash::Dash, pages::discover::Discover, pages::home::Home, pages::member::Member,
    pages::not_found::PageNotFound, pages::success::Success,
};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[layout(Onboard)]
        #[route("/discover")]
        Discover {},
        #[route("/custom")]
        Custom {},
        #[layout(Authenticated)]
            #[route("/dash")]
            Dash {},
            #[route("/member")]
            Member {},
        #[end_layout]
    #[end_layout]
    #[route("/success")]
    Success {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
