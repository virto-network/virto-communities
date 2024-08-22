use dioxus::prelude::*;
use crate::components::atoms::dropdown::DropdownItem;
#[derive(Clone, Debug, Default)]
pub struct Community {
    pub id: String,
    pub name: String,
    pub industry: DropdownItem,
    pub description: String,
    pub logo: Option<String>,
}
pub type Communities = Vec<Community>;
pub fn use_communities() -> UseCommunitiesState {
    let communities = consume_context::<Signal<Communities>>();
    let community = consume_context::<Signal<Community>>();
    use_hook(|| UseCommunitiesState {
        inner: communities,
        community,
    })
}
#[derive(Clone, Copy)]
pub struct UseCommunitiesState {
    inner: Signal<Communities>,
    community: Signal<Community>,
}
impl UseCommunitiesState {
    pub fn get(&self) -> Communities {
        self.inner.read().clone()
    }
    pub fn set(&mut self, communities: Communities) {
        let mut inner = self.inner.write();
        *inner = communities;
    }
    pub fn push(&mut self, community: Community) {
        let mut inner = self.inner.write();
        inner.push(community);
    }
    pub fn get_community(&self) -> Community {
        self.community.read().clone()
    }
    pub fn set_community(&mut self, community: Community) {
        let mut c = self.community.write();
        *c = community;
    }
    pub fn default(&mut self) {
        self.set(Communities::default())
    }
}
