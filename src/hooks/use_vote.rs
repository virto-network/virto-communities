use dioxus::prelude::*;
#[derive(Clone, Debug)]
pub enum ProposalStatus {
    APPROVED,
    REJECTED,
    VOTING,
    QUEUE,
}
#[derive(Clone, Debug)]
pub enum BadgeColor {
    YELLOW,
    RED,
    GREEN,
}
#[derive(Clone, Debug, Default)]
pub struct VoteDigest {
    pub aye: u64,
    pub nay: u64,
}
impl VoteDigest {
    pub fn total(&self) -> u64 {
        self.aye + self.nay
    }
    pub fn percent_aye(&self) -> f64 {
        if self.total() > 0 {
            let percent_unit = 100.0 / self.total() as f64;
            percent_unit * self.aye as f64
        } else {
            50.0
        }
    }
    pub fn percent_nay(&self) -> f64 {
        if self.total() > 0 {
            let percent_unit = 100.0 / self.total() as f64;
            percent_unit * self.nay as f64
        } else {
            50.0
        }
    }
}
pub fn use_vote() -> UseVoteState {
    use_hook(move || UseVoteState {})
}
#[derive(Clone, Copy)]
pub struct UseVoteState {}
impl UseVoteState {}
