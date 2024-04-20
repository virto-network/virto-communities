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

    use_hook(|| UseCommunitiesState { inner: communities })
}

#[derive(Clone, Copy)]
pub struct UseCommunitiesState {
    inner: Signal<Communities>,
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

    pub fn default(&mut self) {
        self.set(Communities::default())
    }
}
