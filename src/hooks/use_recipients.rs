use std::vec;

use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use super::{
    use_recipient::RecipientForm,
    use_tooltip::{use_tooltip, TooltipItem},
};

pub type Recipients = Vec<RecipientForm>;
pub fn use_recipients() -> UseRecipientsState {
    let i18 = use_i18();
    let mut tooltip = use_tooltip();

    let recipients = consume_context::<Signal<Recipients>>();
    let recipient = consume_context::<Signal<RecipientForm>>();
    let mut is_loading = use_signal(|| false);

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        tooltip.handle_tooltip(TooltipItem {
            title: translate!(i18, "dashboard.tips.loading.title"),
            body: translate!(i18, "dashboard.tips.loading.description"),
            show: true,
        });

        tooltip.hide();
        is_loading.set(false);
    });

    use_hook(|| UseRecipientsState {
        recipients,
        recipient,
        is_loading,
    })
}
#[derive(Clone, Copy)]
pub struct UseRecipientsState {
    recipients: Signal<Recipients>,
    recipient: Signal<RecipientForm>,
    pub is_loading: Signal<bool>,
}

pub enum RecipientsError {
    NotFound,
    FailedUpdatingFavorites,
    NotFoundFavorite,
}

impl UseRecipientsState {
    pub fn get_recipients(&self) -> Vec<RecipientForm> {
        self.recipients.read().clone()
    }

    pub fn get_recipients_by_filters(
        &self,
        filter_by_name: Option<&str>,
        filter_by_pagination: Option<(usize, usize)>,
    ) -> Vec<RecipientForm> {
        if self.is_loading.read().clone() {
            return vec![];
        }

        let recipients = self.recipients.read().clone();

        match filter_by_name {
            Some(name) => recipients
                .into_iter()
                .filter(|recipient| recipient.alias.to_lowercase().contains(&name))
                .collect::<Vec<RecipientForm>>(),
            None => {
                if let Some((from, to)) = filter_by_pagination {
                    if to > recipients.len() {
                        recipients[from..recipients.len()].to_vec()
                    } else {
                        recipients[from..to].to_vec()
                    }
                } else {
                    recipients
                }
            }
        }
    }
    pub fn push_recipient(&mut self, recipient: RecipientForm) {
        self.recipients.push(recipient);
    }
    pub fn set_recipient(&mut self, id: u16) -> Result<(), RecipientsError> {
        let mut c = self.recipient.write();

        let position = self
            .recipients
            .read()
            .iter()
            .position(|recipient| recipient.id == id)
            .ok_or(RecipientsError::NotFound)?;

        *c = self.recipients.read()[position].clone();
        Ok(())
    }

    pub fn remove_recipient(&mut self) {
        let mut c = self.recipient.write();
        *c = RecipientForm::default()
    }
    pub fn get_recipient(&self) -> RecipientForm {
        self.recipient.read().clone()
    }
}
