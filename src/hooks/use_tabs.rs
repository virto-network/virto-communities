use dioxus::prelude::*;
#[derive(Clone, Debug)]
pub struct TabCommunity {
    id: u16,
    tabs: Vec<Tab>,
}
#[derive(Clone, Debug)]
pub struct Tab {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct TabSelected(Option<usize>);

pub fn use_tabs() -> UsetabsState {
    let tabs = consume_context::<Signal<Vec<Tab>>>();
    let selected = consume_context::<Signal<TabSelected>>();
    use_hook(|| UsetabsState {
        inner: tabs,
        selected,
    })
}
#[derive(Clone, Copy)]
pub struct UsetabsState {
    inner: Signal<Vec<Tab>>,
    selected: Signal<TabSelected>,
}
impl UsetabsState {
    pub fn get(&self) -> Vec<Tab> {
        self.inner.read().clone()
    }
    pub fn set(&mut self, tabs: Vec<Tab>) {
        let mut inner = self.inner.write();
        *inner = tabs;
    }
    pub fn push(&mut self, tab: Tab) {
        let mut s = self.clone();
        self.inner.with_mut(|i| i.push(tab));
        if let Some(_) = self.inner.read().get(self.len() - 1) {
            s.select(Some(self.len() - 1));
        }
    }
    pub fn len(&self) -> usize {
        self.inner.read().len()
    }
    pub fn update(&mut self, tab: Tab) {
        if let Some(index) = self.selected() {
            self.inner.with_mut(|i| i[index] = tab)
        }
    }
    /// Delete a tab and if the deleted tab is different to selected
    /// returns the last tab in the list
    pub fn remove(&mut self, index: usize) -> Option<Tab> {
        let mut s = self.clone();
        self.inner.with_mut(|i| i.remove(index));

        if let Some(i) = self.selected() {
            if index == i {
                s.select(None);
            }
        }

        if self.len() == 0 {
            s.select(None);
        } else {
            let latest = self.len() - 1;
            return self.inner.read().get(latest).cloned();
        }

        None
    }
    pub fn select(&mut self, index: Option<usize>) {
        let mut selected = self.selected.write();
        selected.0 = index;
    }
    pub fn selected(&self) -> Option<usize> {
        self.selected.read().0
    }
}
