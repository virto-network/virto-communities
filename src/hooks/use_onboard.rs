use dioxus::prelude::*;
#[derive(Clone, Default, Debug)]
pub struct BasicsForm {
    pub logo: Option<String>,
    pub name: String,
    pub description: String,
    pub industry: String,
}
#[derive(Clone, Default, Debug)]
pub enum ManagementOptions {
    #[default]
    Membership,
    Ranked,
}
#[derive(Clone, Default, Debug)]
pub struct ManagementForm {
    pub value: ManagementOptions,
}
#[derive(Clone, Default, Debug)]
pub enum MediumOptions {
    #[default]
    Wallet,
}
#[derive(Clone, Default, Debug)]
pub struct InvitationItem {
    pub medium: MediumOptions,
    pub account: String,
}
pub type Invitations = Vec<InvitationItem>;
#[derive(Clone, Default, Debug)]
pub struct InvitationForm {
    pub invitations: Invitations,
}
pub fn use_onboard() -> UseOnboardState {
    let basics = consume_context::<Signal<BasicsForm>>();
    let management = consume_context::<Signal<ManagementForm>>();
    let invitations = consume_context::<Signal<InvitationForm>>();
    use_hook(|| UseOnboardState {
        inner: UseOnboardInner {
            basics,
            management,
            invitations,
        },
    })
}
#[derive(Clone, Copy)]
pub struct UseOnboardState {
    inner: UseOnboardInner,
}
#[derive(Clone, Copy, Default)]
pub struct UseOnboardInner {
    basics: Signal<BasicsForm>,
    management: Signal<ManagementForm>,
    invitations: Signal<InvitationForm>,
}
impl UseOnboardState {
    pub fn get(&self) -> UseOnboardInner {
        self.inner.clone()
    }
    pub fn get_basics(&self) -> BasicsForm {
        self.inner.basics.read().clone()
    }
    pub fn set_basics(&mut self, basics: BasicsForm) {
        let mut inner = self.inner.basics.write();
        *inner = basics;
    }
    pub fn basics_mut(&mut self) -> Signal<BasicsForm> {
        self.inner.basics.clone()
    }
    pub fn get_management(&self) -> ManagementForm {
        self.inner.management.read().clone()
    }
    pub fn management_mut(&mut self) -> Signal<ManagementForm> {
        self.inner.management
    }
    pub fn get_invitations(&self) -> Invitations {
        self.inner.invitations.read().invitations.clone()
    }
    pub fn set_management(&mut self, management: ManagementForm) {
        let mut inner = self.inner.management.write();
        *inner = management;
    }
    pub fn set_invitations(&mut self, invitations: InvitationForm) {
        let mut inner = self.inner.invitations.write();
        *inner = invitations;
    }
    pub fn push_invitation(&mut self, invitation: InvitationItem) {
        self.inner.invitations.with_mut(|i| i.invitations.push(invitation));
    }
    pub fn remove_invitation(&mut self, position: usize) {
        self.inner.invitations.with_mut(|i| i.invitations.remove(position));
    }
    pub fn update_invitation(&mut self, position: usize, invitation: InvitationItem) {
        self.inner.invitations.with_mut(|i| i.invitations[position] = invitation);
    }
    pub fn default(&mut self) {
        self.inner = UseOnboardInner::default();
    }
}
