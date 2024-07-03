#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};
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
        use_timestamp::{use_timestamp, IsTimestampHandled, TimestampValue},
        use_tooltip::use_tooltip,
    },
    pages::route::Route,
    services::kreivo::timestamp::now,
};

fn main() {
    dioxus_logger::init(LevelFilter::Debug).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    use_language();
    use_startup();

    let i18 = use_i18();
    let theme = use_theme();
    let mut notification = use_notification();
    let tooltip = use_tooltip();
    let mut session = use_session();
    let mut timestamp = use_timestamp();
    let mut is_timestamp_handled = consume_context::<Signal<IsTimestampHandled>>();

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        let Ok(result_now) = now().await else {
            notification.handle_error(&translate!(i18, "errors.timestamp.query_failed"));
            is_timestamp_handled.set(IsTimestampHandled(true));
            return;
        };

        timestamp.set(TimestampValue(result_now));
        is_timestamp_handled.set(IsTimestampHandled(true));
    });

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
