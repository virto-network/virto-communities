#![allow(non_snake_case)]

use dioxus::prelude::*;
use gloo::storage::{errors::StorageError, LocalStorage};

use log::LevelFilter;
use virto_communities::{
    components::atoms::{Notification, Tooltip},
    hooks::{
        use_language::use_language,
        use_notification::{use_notification, NotificationHandler},
        use_session::{use_session, UserSession},
        use_startup::use_startup,
        use_theme::use_theme,
        use_tooltip::use_tooltip,
    },
    pages::route::Route,
};

fn main() {
    dioxus_logger::init(LevelFilter::Debug).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    use_language();
    use_startup();
    let theme = use_theme();
    let notification = use_notification();
    let tooltip = use_tooltip();
    let mut session = use_session();

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        let serialized_session: Result<String, StorageError> =
            <LocalStorage as gloo::storage::Storage>::get("session_file");

        let Ok(serialized_session) = serialized_session else {
            return;
        };

        let Ok(user_session) = serde_json::from_str::<UserSession>(&serialized_session) else {
            return;
        };

        session.set(&user_session);
    });

    rsx! {
        div { style: theme.get_style(),
            if notification.get().show {
                Notification {
                    title: "{notification.get().title}",
                    body: "{notification.get().body}",
                    variant: notification.get().variant,
                    on_click: move |_| {
                        match notification.get().handle.value {
                            NotificationHandler::Click => {}
                            NotificationHandler::None => {}
                        }
                    }
                }
            }

            if tooltip.get().show {
                Tooltip {
                    title: "{tooltip.get().title}",
                    body: "{tooltip.get().body}",
                }
            }

            Router::<Route> {}
        }
    }
}
