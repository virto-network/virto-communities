use dioxus::prelude::*;
use dioxus_i18n::t;
#[derive(Debug, PartialEq, Clone, Default)]
pub enum NotificationVariant {
    Warning,
    Error,
    #[default]
    Success,
}
#[derive(Debug, Clone, Default)]
pub struct NotificationItem {
    pub title: String,
    pub body: String,
    pub variant: NotificationVariant,
    pub show: bool,
    pub handle: NotificationHandle,
}
#[derive(Debug, Clone, Default)]
pub struct NotificationHandle {
    pub value: NotificationHandler,
}
#[derive(Debug, Clone, Default)]
pub enum NotificationHandler {
    Click,
    #[default]
    None,
}
pub fn use_notification() -> UseNotificationState {
    let notification = consume_context::<Signal<NotificationItem>>();
    use_hook(move || UseNotificationState {
        inner: notification,
    })
}
#[derive(Clone, Copy)]
pub struct UseNotificationState {
    inner: Signal<NotificationItem>,
}
use gloo::timers::future::TimeoutFuture;
impl UseNotificationState {
    pub fn get(&self) -> NotificationItem {
        self.inner.read().clone()
    }
    pub fn handle_notification(&mut self, item: NotificationItem) {
        let mut inner = self.inner;
        *inner.write() = item;
        spawn_forever(async move {
            TimeoutFuture::new(3000).await;
            *inner.write() = NotificationItem::default();
        });
    }
    pub fn handle_success(&mut self, body: &str) {
        self.handle_notification(NotificationItem {
            title: t!("success-title"),
            body: String::from(body),
            variant: NotificationVariant::Success,
            show: true,
            handle: NotificationHandle {
                value: NotificationHandler::None,
            },
        });
    }
    pub fn handle_error(&mut self, body: &str) {
        self.handle_notification(NotificationItem {
            title: String::from("Error"),
            body: String::from(body),
            variant: NotificationVariant::Error,
            show: true,
            handle: NotificationHandle {
                value: NotificationHandler::None,
            },
        });
    }
    pub fn handle_warning(&mut self, title: &str, body: &str) {
        self.handle_notification(NotificationItem {
            title: String::from(title),
            body: String::from(body),
            variant: NotificationVariant::Warning,
            show: true,
            handle: NotificationHandle {
                value: NotificationHandler::None,
            },
        });
    }
    pub fn clear(&mut self) {
        let mut inner = self.inner.write();
        *inner = NotificationItem::default();
    }
}
