use dioxus::prelude::*;
#[derive(Clone, Default)]
pub struct Paginator {
    pub from: u8,
    pub total: u8,
    pub current: u8,
}
pub fn use_paginator() -> UsePaginatorState {
    let paginator = consume_context::<Signal<Paginator>>();
    use_hook(|| UsePaginatorState {
        inner: paginator,
    })
}
#[derive(Clone, Copy)]
pub struct UsePaginatorState {
    inner: Signal<Paginator>,
}
impl UsePaginatorState {
    pub fn get(&self) -> Paginator {
        self.inner.read().clone()
    }
    pub fn set(&mut self, paginator: Paginator) {
        let mut inner = self.inner.write();
        *inner = paginator;
    }
    pub fn set_from(&mut self, from: u8) {
        let mut inner = self.inner.write();
        inner.from = from;
    }
    pub fn set_total(&mut self, total: u8) {
        let mut inner = self.inner.write();
        inner.total = total;
    }
    pub fn set_current(&mut self, current: u8) {
        let mut inner = self.inner.write();
        inner.current = current;
    }
    pub fn default(&mut self) {
        self.set(Paginator::default())
    }
}
