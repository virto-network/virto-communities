use dioxus::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct TooltipItem {
    pub title: String,
    pub body: String,
    pub show: bool,
}

pub fn use_tooltip() -> UseTooltipState {
    let tooltip = consume_context::<Signal<TooltipItem>>();

    use_hook(move || UseTooltipState {
        inner: tooltip,
    })
}

#[derive(Clone, Copy)]
pub struct UseTooltipState {
    inner: Signal<TooltipItem>,
}

impl UseTooltipState {
    pub fn get(&self) -> TooltipItem {
        self.inner.read().clone()
    }

    pub fn handle_tooltip(&mut self, item: TooltipItem) {
        let mut this = self.clone();
        let mut inner = self.inner.clone();
        *inner.write() = item;
    }

    pub fn hide(&mut self) {
        let mut inner = self.inner.write();
        *inner = TooltipItem::default();
    }
}
