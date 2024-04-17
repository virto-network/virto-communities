#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::Router;

use log::LevelFilter;
use virto_communities::{hooks::use_language::use_language, pages::route::Route};

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    use_language();
    
    rsx! {
        Router::<Route> {}
    }
}
