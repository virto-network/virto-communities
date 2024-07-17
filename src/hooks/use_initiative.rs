use dioxus::prelude::*;

use crate::components::atoms::dropdown::DropdownItem;

#[derive(Clone, Default, Debug)]
pub struct InfoForm {
    pub name: String,
    pub description: String,
    pub categories: String,
}

#[derive(Clone, Default, Debug)]
pub enum MediumOptions {
    #[default]
    Wallet,
    Telegram,
    Email,
}

#[derive(Clone, Default, Debug)]
pub struct MemberItem {
    pub medium: MediumOptions,
    pub account: String,
}

pub type Members = Vec<MemberItem>;

#[derive(Clone, Default, Debug)]
pub struct Treasury {
    pub amount: u128,
    pub dest: String,
}

#[derive(Clone, Debug, Default)]
pub struct AddMembersAction {
    pub members: Members,
}

impl AddMembersAction {
    pub fn add_member(&mut self, member: MemberItem) {
        self.members.push(member);
    }

    pub fn update_member(&mut self, index: usize, member: MemberItem) {
        if index < self.members.len() {
            self.members[index] = member;
        } else {
            println!("Index out of bounds.");
        }
    }

    pub fn remove_member(&mut self, index: usize) {
        if index < self.members.len() {
            self.members.remove(index);
        } else {
            println!("Index out of bounds.");
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RemoveMembersAction {
    pub members: Members,
}

impl RemoveMembersAction {
    pub fn add_member(&mut self, member: MemberItem) {
        self.members.push(member);
    }

    pub fn update_member(&mut self, index: usize, member: MemberItem) {
        if index < self.members.len() {
            self.members[index] = member;
        } else {
            println!("Index out of bounds.");
        }
    }

    pub fn remove_member(&mut self, index: usize) {
        if index < self.members.len() {
            self.members.remove(index);
        } else {
            println!("Index out of bounds.");
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TreasuryAction {
    pub treasury: Treasury,
}

impl TreasuryAction {
    pub fn update_amount(&mut self, amount: u128) {
        self.treasury.amount = amount;
    }

    pub fn update_dest(&mut self, dest: String) {
        self.treasury.dest = dest;
    }
}

#[derive(Clone, Debug)]
pub enum ActionItem {
    AddMembers(AddMembersAction),
    RemoveMembers(RemoveMembersAction),
    Treasury(TreasuryAction),
}

impl ActionItem {
    pub fn option(&self) -> DropdownItem {
        match self {
            ActionItem::AddMembers(_) => DropdownItem {
                key: "AddMembers".to_string(),
                value: "Add Members".to_string(),
            },
            ActionItem::RemoveMembers(_) => DropdownItem {
                key: "RemoveMembers".to_string(),
                value: "Remove Members".to_string(),
            },
            ActionItem::Treasury(_) => DropdownItem {
                key: "Treasury".to_string(),
                value: "Treasury".to_string(),
            },
        }
    }

    fn to_option(option: String) -> ActionItem {
        match &*option {
            "AddMembers" => ActionItem::AddMembers(AddMembersAction::default()),
            "RemoveMembers" => ActionItem::RemoveMembers(RemoveMembersAction::default()),
            "Treasury" => ActionItem::Treasury(TreasuryAction::default()),
            _ => todo!(),
        }
    }

    fn get_options() -> Vec<DropdownItem> {
        vec![
            ActionItem::AddMembers(AddMembersAction::default()).option(),
            ActionItem::RemoveMembers(RemoveMembersAction::default()).option(),
            ActionItem::Treasury(TreasuryAction::default()).option(),
        ]
    }
}

#[derive(Clone, Default, Debug)]
pub struct ActionsForm {
    pub value: Vec<ActionItem>,
}

#[derive(Clone, Default, Debug)]
pub struct SettingsForm {}

#[derive(Clone, Default, Debug)]
pub struct ConfirmationForm {}

pub fn use_initiative() -> UseInitiativeState {
    let info = consume_context::<Signal<InfoForm>>();
    let actions = consume_context::<Signal<ActionsForm>>();
    let settings = consume_context::<Signal<SettingsForm>>();
    let confirmation = consume_context::<Signal<ConfirmationForm>>();

    use_hook(|| UseInitiativeState {
        inner: UseInitiativeInner {
            info,
            actions,
            settings,
            confirmation,
        },
    })
}

#[derive(Clone, Copy)]
pub struct UseInitiativeState {
    inner: UseInitiativeInner,
}

#[derive(Clone, Copy, Default)]
pub struct UseInitiativeInner {
    info: Signal<InfoForm>,
    actions: Signal<ActionsForm>,
    settings: Signal<SettingsForm>,
    confirmation: Signal<ConfirmationForm>,
}

impl UseInitiativeState {
    pub fn get(&self) -> UseInitiativeInner {
        self.inner.clone()
    }

    pub fn get_info(&self) -> InfoForm {
        self.inner.info.read().clone()
    }

    pub fn set_info(&mut self, info: InfoForm) {
        let mut inner = self.inner.info.write();
        *inner = info;
    }

    pub fn info_mut(&mut self) -> Signal<InfoForm> {
        self.inner.info.clone()
    }

    pub fn get_actions(&self) -> Vec<ActionItem> {
        self.inner.actions.read().value.clone()
    }

    pub fn get_action(&self, position: usize) -> ActionItem {
        self.inner.actions.read().value[position].clone()
    }

    pub fn get_actions_options(&self) -> Vec<DropdownItem> {
        ActionItem::get_options()
    }

    pub fn to_action_option(&self, option: String) -> ActionItem {
        ActionItem::to_option(option)
    }

    pub fn set_actions(&mut self, actions: ActionsForm) {
        let mut inner = self.inner.actions.write();
        *inner = actions;
    }

    pub fn push_action(&mut self, action: ActionItem) {
        self.inner.actions.with_mut(|i| i.value.push(action));
    }

    pub fn remove_action(&mut self, position: usize) {
        self.inner.actions.with_mut(|i| i.value.remove(position));
    }

    pub fn update_action(&mut self, position: usize, action: ActionItem) {
        self.inner.actions.with_mut(|i| i.value[position] = action);
    }

    pub fn get_settings(&self) -> SettingsForm {
        self.inner.settings.read().clone()
    }

    pub fn set_settings(&mut self, settings: SettingsForm) {
        let mut inner = self.inner.settings.write();
        *inner = settings;
    }

    pub fn settings_mut(&mut self) -> Signal<SettingsForm> {
        self.inner.settings
    }

    pub fn get_confirmation(&self) -> ConfirmationForm {
        self.inner.confirmation.read().clone()
    }

    pub fn set_confirmation(&mut self, confirmation: ConfirmationForm) {
        let mut inner = self.inner.confirmation.write();
        *inner = confirmation;
    }

    pub fn confirmation_mut(&mut self) -> Signal<ConfirmationForm> {
        self.inner.confirmation
    }

    pub fn default(&mut self) {
        self.inner = UseInitiativeInner::default()
    }
}
