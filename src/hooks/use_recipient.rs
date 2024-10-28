use dioxus::prelude::*;
#[derive(Default, PartialEq, Debug, Clone)]
pub enum RecipientType {
    #[default]
    Person,
    Business,
    Dao,
    Initiative,
}
#[derive(Clone, Default, Debug)]
pub struct RecipientForm {
    pub id: u16,
    pub name: String,
    pub email: String,
    pub alias: String,
    pub country: String,
    pub state: String,
    pub address: String,
    pub address_complement: String,
    pub address_context: String,
    pub city: String,
    pub zip: String,
    pub details: RecipientType,
}
pub fn use_recipient() -> UseRecipientState {
    let recipient = consume_context::<Signal<RecipientForm>>();
    use_hook(|| UseRecipientState {
        inner: UseRecipientInner { recipient },
    })
}
#[derive(Clone, Copy)]
pub struct UseRecipientState {
    inner: UseRecipientInner,
}
#[derive(Clone, Copy, Default)]
pub struct UseRecipientInner {
    recipient: Signal<RecipientForm>,
}
impl UseRecipientState {
    pub fn get(&self) -> UseRecipientInner {
        self.inner.clone()
    }
    pub fn get_recipient(&self) -> RecipientForm {
        self.inner.recipient.read().clone()
    }
    pub fn set_recipient(&mut self, recipient: RecipientForm) {
        let mut inner = self.inner.recipient.write();
        *inner = recipient;
    }
    pub fn recipient_mut(&mut self) -> Signal<RecipientForm> {
        self.inner.recipient.clone()
    }
    pub fn default(&mut self) {
        self.inner = UseRecipientInner::default();
    }
}
