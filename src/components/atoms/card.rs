use dioxus::prelude::*;

use crate::{components::atoms::{
    avatar::Variant, ArrowRight, Avatar, Icon, IconButton, Suitcase, UserGroup,
}, pages::dashboard::Community};

#[derive(PartialEq, Props, Clone)]
pub struct CardProps {
    community: Community,
}

pub fn Card(props: CardProps) -> Element {
    rsx!(
        section { class: "card",
            div { class: "card__container",
                div { class: "card__head",
                    IconButton {
                        body: rsx!(
                            Avatar {
                                name: "{props.community.name}",
                                size: 48,
                                uri: props.community.icon,
                                variant: Variant::SemiRound
                            }
                        ),
                        on_click: move |_| { }
                    }
                    h3 { class: "card__title",
                        "{props.community.name}"
                    }
                }
                p { class: "card__description",
                    "{props.community.description}"
                }
                div { class: "card__metrics",
                    span { class: "card__metric",
                        Icon {
                            icon: UserGroup,
                            height: 16,
                            width: 16,
                            stroke_width: 1,
                            fill: "var(--white)"
                        }
                        small {
                            "{props.community.memberships}"
                        }
                    }
                    span { class: "card__metric",
                        Icon {
                            icon: Suitcase,
                            height: 16,
                            width: 16,
                            stroke_width: 1,
                            fill: "var(--white)"
                        }
                        small {
                            "{props.community.members}"
                        }
                    }
                }
            }

            div { class: "card__cta",
                IconButton {
                    class: "button--avatar",
                    body: rsx!(
                        Icon {
                            icon: ArrowRight,
                            height: 16,
                            width: 16,
                            stroke_width: 2,
                            stroke: "var(--text-primary)"
                        }
                    ),
                    on_click: move |_| { }
                }
            }
        }
    )
}

