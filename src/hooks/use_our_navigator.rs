use dioxus::prelude::*;
#[derive(Clone, Default)]
pub struct FromMiddleware(pub bool);
pub fn use_our_navigator() -> UseOurNavigatorState {
    let nav = use_navigator();
    use_hook(|| UseOurNavigatorState { nav })
}
#[derive(Clone, Copy)]
pub struct UseOurNavigatorState {
    nav: Navigator,
}
impl UseOurNavigatorState {
    pub fn push(
        &self,
        middlewares: Vec<Box<dyn FnOnce() -> Result<(), &'static str>>>,
        to: &str,
    ) {
        for middleware in middlewares {
            if let Err(e) = middleware() {
                println!("Middleware failed: {}", e);
                return;
            }
        }
        self.nav.push(to);
    }
    pub fn can_go_back(&self) -> bool {
        self.nav.can_go_back()
    }
    pub fn go_back(&self) {
        self.nav.go_back()
    }
}
