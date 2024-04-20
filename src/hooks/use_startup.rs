use dioxus::{hooks::use_context_provider, signals::Signal};

use super::{use_attach::AttachFile, use_communities::Communities, use_onboard::OnboardForm};

pub fn use_startup() {
    use_context_provider::<Signal<OnboardForm>>(|| Signal::new(OnboardForm::default()));
    use_context_provider::<Signal<Communities>>(|| Signal::new(vec![]));
    use_context_provider::<Signal<Option<AttachFile>>>(|| Signal::new(None));
}
