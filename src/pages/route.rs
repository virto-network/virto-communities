use dioxus::prelude::*;

use crate::{
    pages::custom::Custom, pages::dash::Dash, pages::discover::Discover, pages::home::Home,
    pages::member::Member,
};

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/discover")]
    Discover {},
    #[route("/custom")]
    Custom {},
    #[route("/dash")]
    Dash {},
    #[route("/member")]
    Member {},
}