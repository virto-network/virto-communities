use crate::components::atoms::dropdown::DropdownItem;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct InfoForm {
    pub name: String,
    pub description: String,
    pub categories: Vec<String>,
}
#[derive(PartialEq, Clone, Default, Deserialize, Serialize, Debug)]
pub enum MediumOptions {
    #[default]
    Wallet,
}
#[derive(PartialEq, Clone, Default, Deserialize, Serialize, Debug)]
pub struct MemberItem {
    pub medium: MediumOptions,
    pub account: String,
}
pub type Members = Vec<MemberItem>;
#[derive(PartialEq, Clone, Default, Deserialize, Serialize, Debug)]
pub struct KusamaTreasury {
    pub date: String,
    pub amount: u64,
}
#[derive(PartialEq, Clone, Default, Deserialize, Serialize, Debug)]
pub struct KusamaTreasuryPeriod {
    pub blocks: Option<u64>,
    pub amount: u64,
}
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
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
pub type KusamaTreasuryPeriods = Vec<KusamaTreasury>;
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct KusamaTreasuryAction {
    pub periods: KusamaTreasuryPeriods,
}
impl KusamaTreasuryAction {
    pub fn add_period(&mut self, period: KusamaTreasury) {
        self.periods.push(period);
    }
    pub fn update_period(&mut self, index: usize, period: KusamaTreasury) {
        if index < self.periods.len() {
            self.periods[index] = period;
        } else {
            println!("Index out of bounds.");
        }
    }
    pub fn remove_period(&mut self, index: usize) {
        if index < self.periods.len() {
            self.periods.remove(index);
        } else {
            println!("Index out of bounds.");
        }
    }
}
#[derive(PartialEq, Clone, Default, Deserialize, Serialize, Debug)]
pub enum ConvictionVote {
    #[default]
    None,
    Locked1x,
    Locked2x,
    Locked3x,
    Locked4x,
    Locked5x,
    Locked6x,
}
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct StandardVote {
    pub aye: bool,
    pub conviction: ConvictionVote,
    pub balance: u64,
}
#[derive(PartialEq, Clone, Deserialize, Serialize, Debug)]
pub enum VoteType {
    Standard(StandardVote),
}
impl Default for VoteType {
    fn default() -> Self {
        VoteType::Standard(StandardVote {
            aye: true,
            conviction: ConvictionVote::None,
            balance: 0,
        })
    }
}
impl VoteType {
    pub fn key_string(&self) -> &str {
        match *self {
            VoteType::Standard(_) => "Standard",
        }
    }
}
#[derive(PartialEq, Clone, Default, Deserialize, Serialize, Debug)]
pub struct VotingOpenGov {
    pub poll_index: u64,
    pub vote: VoteType,
}
impl VotingOpenGov {
    pub fn serialize_vote_type(&self) -> serde_json::Value {
        match &self.vote {
            VoteType::Standard(vote) => {
                serde_json::json!(
                    { "pollIndex" : self.poll_index, "vote" : { "type" : "Standard",
                    "aye" : vote.aye, "conviction" : vote.conviction, "balance" : vote
                    .balance } }
                )
            }
        }
    }
}
pub type VotingOpenGovActionProposals = Vec<VotingOpenGov>;
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct VotingOpenGovAction {
    pub proposals: VotingOpenGovActionProposals,
}
impl VotingOpenGovAction {
    pub fn add_proposal(&mut self, proposal: VotingOpenGov) {
        self.proposals.push(proposal);
    }
    pub fn update_proposal(&mut self, index: usize, proposal: VotingOpenGov) {
        if index < self.proposals.len() {
            self.proposals[index] = proposal;
        } else {
            println!("Index out of bounds.");
        }
    }
    pub fn remove_proposal(&mut self, index: usize) {
        if index < self.proposals.len() {
            self.proposals.remove(index);
        } else {
            println!("Index out of bounds.");
        }
    }
}
#[derive(PartialEq, Clone, Default, Deserialize, Serialize, Debug)]
pub struct TransferItem {
    pub account: String,
    pub value: u64
}
pub type Transfers = Vec<TransferItem>;
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct CommunityTransferAction {
    pub transfers: Transfers,
}
impl CommunityTransferAction {
    pub fn add_transfer(&mut self, transfer: TransferItem) {
        self.transfers.push(transfer);
    }
    pub fn update_transfer(&mut self, index: usize, transfer: TransferItem) {
        if index < self.transfers.len() {
            self.transfers[index] = transfer;
        } else {
            println!("Index out of bounds.");
        }
    }
    pub fn remove_transfer(&mut self, index: usize) {
        if index < self.transfers.len() {
            self.transfers.remove(index);
        } else {
            println!("Index out of bounds.");
        }
    }
}
#[derive(PartialEq, Clone, Deserialize, Serialize, Debug)]
#[serde(tag = "action_type")]
pub enum ActionItem {
    AddMembers(AddMembersAction),
    KusamaTreasury(KusamaTreasuryAction),
    VotingOpenGov(VotingOpenGovAction),
    CommunityTransfer(CommunityTransferAction),
}
impl ActionItem {
    pub fn option(&self) -> DropdownItem {
        match self {
            ActionItem::AddMembers(_) => DropdownItem {
                key: "AddMembers".to_string(),
                value: "Add Members".to_string(),
            },
            ActionItem::KusamaTreasury(_) => DropdownItem {
                key: "KusamaTreasury".to_string(),
                value: "Kusama - Request treasury spend".to_string(),
            },
            ActionItem::VotingOpenGov(_) => DropdownItem {
                key: "VotingOpenGov".to_string(),
                value: "Kusama - Vote in OpenGov".to_string(),
            },
            ActionItem::CommunityTransfer(_) => DropdownItem {
                key: "CommunityTransfer".to_string(),
                value: "Community Transfer".to_string(),
            },
        }
    }
    fn to_option(option: String) -> ActionItem {
        match &*option {
            "AddMembers" => ActionItem::AddMembers(AddMembersAction::default()),
            "KusamaTreasury" => ActionItem::KusamaTreasury(KusamaTreasuryAction::default()),
            "VotingOpenGov" => ActionItem::VotingOpenGov(VotingOpenGovAction::default()),
            "CommunityTransfer" => ActionItem::CommunityTransfer(CommunityTransferAction::default()),
            _ => todo!(),
        }
    }
    fn get_options() -> Vec<DropdownItem> {
        vec![
            ActionItem::AddMembers(AddMembersAction::default()).option(),
            ActionItem::KusamaTreasury(KusamaTreasuryAction::default()).option(),
            ActionItem::VotingOpenGov(VotingOpenGovAction::default()).option(),
            ActionItem::CommunityTransfer(CommunityTransferAction::default()).option(),
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
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct InitiativeInitContent {
    pub sender: String,
    pub is_admin: bool,
}
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize, Default)]
pub struct InitiativeInfoContent {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub actions: Vec<ActionItem>,
}
#[derive(PartialEq, Deserialize, Serialize, Debug)]
pub struct InitiativeData {
    pub init: InitiativeInitContent,
    pub info: InitiativeInfoContent,
}
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
pub enum VoteOf {
    Yes,
    No,
}
#[derive(PartialEq, Deserialize, Serialize, Clone, Debug)]
pub enum Vote {
    Standard(VoteOf),
}
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct InitiativeVoteContent {
    pub user: String,
    pub vote: Vote,
}
#[derive(PartialEq, Clone, Debug, Deserialize, Serialize)]
pub struct InitiativeVoteData {
    pub user: String,
    pub room: String,
    pub vote: Vote,
}
#[derive(PartialEq, Deserialize, Serialize, Debug, Default)]
pub struct InitiativeHistory {
    pub init: InitiativeInitContent,
    pub info: InitiativeInfoContent,
    pub votes: Vec<InitiativeVoteContent>,
}
#[derive(PartialEq, Deserialize, Serialize, Debug, Default)]
pub struct CommunityInitiative {
    id: Option<u32>,
}
pub fn use_initiative() -> UseInitiativeState {
    let info = consume_context::<Signal<InfoForm>>();
    let actions = consume_context::<Signal<ActionsForm>>();
    let settings = consume_context::<Signal<SettingsForm>>();
    let confirmation = consume_context::<Signal<ConfirmationForm>>();
    let is_loading = use_signal(|| false);
    use_hook(|| UseInitiativeState {
        inner: UseInitiativeInner {
            info,
            actions,
            settings,
            confirmation,
            is_loading,
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
    is_loading: Signal<bool>,
}
impl UseInitiativeState {
    pub fn is_loading(&self) -> bool {
        self.inner.is_loading.read().clone()
    }
    pub fn set_loading(&mut self, loading: bool) {
        self.inner.is_loading.set(loading);
    }
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
        self.inner = UseInitiativeInner::default();
    }
}
