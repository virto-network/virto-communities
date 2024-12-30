#![allow(non_snake_case)]
use dioxus::{
    logger::tracing::{debug, info, Level},
    prelude::*,
};
use dioxus_i18n::t;
use gloo::storage::{errors::StorageError, LocalStorage};
use virto_communities::{
    components::atoms::{Notification, Tooltip},
    hooks::{
        use_language::use_language,
        use_notification::{use_notification, NotificationHandler},
        use_session::{use_session, UserSession},
        use_startup::use_startup,
        use_timestamp::{use_timestamp, IsTimestampHandled, TimestampValue},
        use_tooltip::use_tooltip,
    },
    pages::route::Route,
    services::kreivo::timestamp::now,
};
const FAVICON: Asset = asset!("/public/favicon.ico");
const MAIN_CSS: Asset = asset!("/public/css-out/main.css");

fn main() {
    let log_level = if cfg!(feature = "staging") {
        Level::DEBUG
    } else {
        Level::INFO
    };
    dioxus::logger::init(log_level);
    launch(App);
}
fn App() -> Element {
    use_language();
    use_startup();
    let mut notification = use_notification();
    let tooltip = use_tooltip();
    let mut session = use_session();
    let mut timestamp = use_timestamp();
    let mut is_timestamp_handled = consume_context::<Signal<IsTimestampHandled>>();
    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        let Ok(result_now) = now().await else {
            notification.handle_error(&t!("errors-timestamp-query_failed"));
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
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
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
            Tooltip { title: "{tooltip.get().title}", body: "{tooltip.get().body}" }
        }
        Router::<Route> {}
    }
}
