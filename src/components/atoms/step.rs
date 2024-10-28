use dioxus::prelude::*;
use crate::components::atoms::{CircleShape, Glyph, Icon};
#[derive(PartialEq, Props, Clone)]
pub struct StepProps {
    #[props(default = false)]
    is_active: bool,
    #[props(default = false)]
    is_completed: bool,
    #[props(default = false)]
    has_cube: bool,
    #[props(default = false)]
    is_column: bool,
    name: Option<String>,
    on_click: EventHandler<MouseEvent>,
}
pub fn Step(props: StepProps) -> Element {
    rsx!(
        button {
            class: "step",
            class: if props.is_active { "step--active" },
            class: if props.is_completed { "step--complete" },
            class: if props.is_column { "step--column" },
            onclick: move |event| props.on_click.call(event),
            if props.has_cube {
                div { class: "step__cube",
                    if props.is_completed {
                        Icon { icon: Glyph, height: 13, width: 16, stroke_width: 2, stroke: "var(--fill-400)" }
                    } else if props.is_active {
                        Icon { icon: CircleShape, height: 12, width: 12, fill: "var(--fill-00)" }
                    } else {
                        Icon {
                            icon: CircleShape,
                            height: 12,
                            width: 12,
                            stroke_width: 2,
                            stroke: "var(--fill-400)"
                        }
                    }
                }
            }
            if let Some(name) = props.name {
                span { class: "step__name", {name} }
            }
            div { class: "step__line" }
        }
    )
}
