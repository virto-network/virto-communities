use dioxus::prelude::*;

use crate::components::molecules::{send::RecipientForm, RecipientSelect};

pub enum RecipientStep {
    Add,
    Select,
}

#[derive(PartialEq, Props, Clone)]
pub struct ControlProps {
    pub on_back: EventHandler,
    pub on_next: EventHandler,
}

pub fn Recipient(props: ControlProps) -> Element {
    let mut recipient_step = use_signal::<RecipientStep>(|| RecipientStep::Select);
    rsx!(match &*recipient_step.read() {
        RecipientStep::Add => {
            rsx!(RecipientForm {
                on_back: move |_| recipient_step.set(RecipientStep::Select),
                on_next: move |_| { props.on_next.call(()) },
            })
        }
        RecipientStep::Select => {
            rsx!(RecipientSelect {
                on_add: move |_| recipient_step.set(RecipientStep::Add),
                on_next: move |_| { props.on_next.call(()) },
            })
        }
    })
}
