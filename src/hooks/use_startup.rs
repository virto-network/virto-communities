use dioxus::{hooks::use_context_provider, signals::Signal};

use crate::services::pjs::Pjs;

use super::{
    use_accounts::Account,
    use_attach::AttachFile,
    use_communities::{Communities, Community},
    use_connect_wallet::Wallet,
    use_notification::NotificationItem,
    use_onboard::OnboardForm,
    use_paginator::Paginator,
    use_session::UserSession,
    use_theme::Theme, use_tooltip::TooltipItem,
};

pub fn use_startup() {
    use_context_provider::<Signal<Theme>>(|| Signal::new(Theme::default()));
    use_context_provider::<Signal<OnboardForm>>(|| Signal::new(OnboardForm::default()));
    use_context_provider::<Signal<Communities>>(|| Signal::new(vec![]));
    use_context_provider::<Signal<Community>>(|| Signal::new(Community::default()));
    use_context_provider::<Signal<Option<AttachFile>>>(|| Signal::new(None));
    use_context_provider::<Signal<NotificationItem>>(|| Signal::new(NotificationItem::default()));
    use_context_provider::<Signal<TooltipItem>>(|| Signal::new(TooltipItem::default()));
    use_context_provider::<Signal<Paginator>>(|| Signal::new(Paginator::default()));

    use_context_provider::<Signal<Option<UserSession>>>(|| Signal::new(None));
    use_context_provider::<Signal<Vec<Account>>>(|| Signal::new(vec![]));
    use_context_provider::<Signal<Option<Account>>>(|| Signal::new(None));
    use_context_provider::<Signal<Option<Wallet>>>(|| Signal::new(None));
    use_context_provider::<Signal<Option<Pjs>>>(|| Signal::new(None));

    use_context_provider::<Signal<bool>>(|| Signal::new(false));
}
