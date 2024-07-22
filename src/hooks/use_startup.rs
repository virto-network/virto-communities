use dioxus::{hooks::use_context_provider, signals::Signal};
use pjs::PjsExtension;

use crate::{pages::initiatives::InitiativeWrapper, services::bot::client::SpacesClient};

use super::{
    use_accounts::{Account, IsDaoOwner},
    use_attach::AttachFile,
    use_communities::{Communities, Community},
    use_initiative::{ActionsForm, CommunityInitiative, ConfirmationForm, InfoForm, SettingsForm},
    use_notification::NotificationItem,
    use_onboard::{BasicsForm, InvitationForm, ManagementForm},
    use_paginator::Paginator,
    use_session::UserSession,
    use_theme::Theme,
    use_tooltip::TooltipItem,
};

const SPACES_CLIENT_URL: &str = "https://bot-api.virto.app";

pub fn use_startup() {
    use_context_provider::<Signal<Theme>>(|| Signal::new(Theme::default()));

    use_context_provider::<Signal<BasicsForm>>(|| Signal::new(BasicsForm::default()));
    use_context_provider::<Signal<ManagementForm>>(|| Signal::new(ManagementForm::default()));
    use_context_provider::<Signal<InvitationForm>>(|| Signal::new(InvitationForm::default()));

    use_context_provider::<Signal<Communities>>(|| Signal::new(vec![]));
    use_context_provider::<Signal<Community>>(|| Signal::new(Community::default()));
    use_context_provider::<Signal<Option<AttachFile>>>(|| Signal::new(None));
    use_context_provider::<Signal<NotificationItem>>(|| Signal::new(NotificationItem::default()));
    use_context_provider::<Signal<TooltipItem>>(|| Signal::new(TooltipItem::default()));
    use_context_provider::<Signal<Paginator>>(|| Signal::new(Paginator::default()));

    use_context_provider::<Signal<Option<UserSession>>>(|| Signal::new(None));
    use_context_provider::<Signal<Vec<Account>>>(|| Signal::new(vec![]));
    use_context_provider::<Signal<Option<Account>>>(|| Signal::new(None));
    use_context_provider::<Signal<IsDaoOwner>>(|| Signal::new(IsDaoOwner(false)));
    use_context_provider::<Signal<Option<PjsExtension>>>(|| Signal::new(None));

    use_context_provider::<Signal<bool>>(|| Signal::new(false));
    use_context_provider::<Signal<String>>(|| Signal::new(String::new()));
    use_context_provider::<Signal<f64>>(|| Signal::new(0.0));

    use_context_provider::<Signal<Option<InitiativeWrapper>>>(|| Signal::new(None));
    use_context_provider::<Signal<InfoForm>>(|| Signal::new(InfoForm::default()));
    use_context_provider::<Signal<ActionsForm>>(|| Signal::new(ActionsForm::default()));
    use_context_provider::<Signal<SettingsForm>>(|| Signal::new(SettingsForm::default()));
    use_context_provider::<Signal<ConfirmationForm>>(|| Signal::new(ConfirmationForm::default()));

    // Clients

    use_context_provider::<Signal<SpacesClient>>(|| {
        Signal::new(SpacesClient::new(SPACES_CLIENT_URL))
    });
}
