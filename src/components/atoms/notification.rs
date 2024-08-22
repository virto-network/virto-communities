use dioxus::prelude::*;
use crate::hooks::use_notification::NotificationVariant;
#[derive(PartialEq, Props, Clone)]
pub struct NotificationProps {
    title: String,
    body: String,
    variant: NotificationVariant,
    on_click: EventHandler<MouseEvent>,
}
pub fn Notification(props: NotificationProps) -> Element {
    let variant = match props.variant {
        NotificationVariant::Warning => "notification--warning",
        NotificationVariant::Error => "notification--error",
        NotificationVariant::Success => "notification--success",
    };
    rsx!(
        button {
            class: "notification {variant}",
            onclick: move |event| props.on_click.call(event),
            h3 { class: "notification__title", "{props.title}" }
            p { class: "notification__body", "{props.body}" }
        }
    )
}
