use dioxus::prelude::*;

#[derive(Clone, Default)]
pub struct OnboardForm {
    pub username: String,
}

pub fn use_onboard() -> UseOnboardState {
    let onboard = consume_context::<Signal<OnboardForm>>();

    use_hook(|| UseOnboardState { inner: onboard })
}

#[derive(Clone, Copy)]
pub struct UseOnboardState {
    inner: Signal<OnboardForm>,
}

impl UseOnboardState {
    pub fn get(&self) -> OnboardForm {
        self.inner.read().clone()
    }

    pub fn set(&mut self, onboard: OnboardForm) {
        let mut inner = self.inner.write();
        *inner = onboard;
    }

    pub fn set_username(&mut self, username: String) {
        let mut inner = self.inner.write();
        inner.username = username;
    }

    pub fn default(&mut self) {
        self.set(OnboardForm::default())
    }
}
