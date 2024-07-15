use dioxus::prelude::*;
use dioxus_std::{i18n::use_i18, translate};

use crate::{
    components::atoms::{
        button::Variant, dropdown::ElementSize, key_value::Variant as KeyValueVariant, Badge,
        Button, KeyValue,
    },
    hooks::{
        use_initiative::use_initiative, use_our_navigator::use_our_navigator,
        use_session::use_session, use_spaces_client::use_spaces_client,
    },
};

#[derive(Clone, Debug)]
pub enum InitiativeStep {
    Info,
    Actions,
    Settings,
    Confirmation,
    None,
}

#[derive(Clone, Debug)]
pub enum ProposalStatus {
    APPROVED,
    REJECTED,
    VOTING,
}

#[derive(Clone, Debug)]
pub enum BadgeColor {
    YELLOW,
    RED,
    GREEN,
}

#[component]
pub fn Vote() -> Element {
    let i18 = use_i18();
    let mut initiative = use_initiative();
    let mut session = use_session();
    let spaces_client = use_spaces_client();
    let mut nav = use_our_navigator();

    let cont = translate!(i18, "utils.vote_preview.description");
    let parser = pulldown_cmark::Parser::new(&cont);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    rsx! {
        div { class: "page--initiative",
            div { class: "initiative__form",
                div { class: "form__wrapper form__wrapper--initiative",
                    h2 { class: "form__title",
                        {translate!(i18, "governance.title")}
                    }
                    div { class: "steps__wrapper",
                        div { class: "row",
                            section { class: "details__proposal",
                                div { class: "vote-card",
                                    div { class: "details__metadata",
                                        KeyValue {
                                            class: "key-value--column",
                                            text: translate!(i18, "governance.description.details.by"),
                                            size: ElementSize::Medium,
                                            variant: KeyValueVariant::Secondary,
                                            body: rsx!(
                                                "Unknown"
                                            )
                                        }
                                        KeyValue {
                                            class: "key-value--column",
                                            text: translate!(i18, "governance.description.details.start-date"),
                                            size: ElementSize::Medium,
                                            variant: KeyValueVariant::Secondary,
                                            body: rsx!(
                                                "15 Jul 2024"
                                            )
                                        }
                                        KeyValue {
                                            class: "key-value--column",
                                            text: translate!(i18, "governance.description.details.end-date"),
                                            size: ElementSize::Medium,
                                            variant: KeyValueVariant::Secondary,
                                            body: rsx!(
                                                "16 Jul 2024"
                                            )
                                        }
                                    }

                                    hr { class: "form__divider" }

                                    div { class: "details__title",
                                        {translate!(i18, "utils.vote_preview.title")}
                                    }

                                    div { class: "details__description markdown-preview",
                                        dangerous_inner_html: "{html_buf}"
                                    }
                                }
                            }

                            section { class: "details__voting",
                                div { class: "vote-card",
                                    KeyValue {
                                        class: "key-value--row",
                                        text: translate!(i18, "governance.description.details.status.title"),
                                        variant: KeyValueVariant::Secondary,
                                        body: {
                                            let status = ProposalStatus::REJECTED;
                                            let (badge_title, badge_color) = match status {
                                                ProposalStatus::APPROVED => (translate!(i18, "governance.description.details.status.options.approved"), "badge--green-dark"),
                                                ProposalStatus::REJECTED => (translate!(i18, "governance.description.details.status.options.rejected"), "badge--red-dark"),
                                                ProposalStatus::VOTING => (translate!(i18, "governance.description.details.status.options.voting"), "badge--lavanda-dark"),
                                            };

                                            rsx!(
                                                Badge {
                                                    text: badge_title,
                                                    class: badge_color.to_string()
                                                }
                                            )
                                        }
                                    }
                                }
                                div { class: "vote-card",
                                    div { class: "details__statistics",
                                        h2 { class: "statistics__title",
                                            {translate!(i18, "governance.description.voting.title")}
                                        }
                                        div { class: "statistics__bar"},
                                        div { class: "statistics__votes",
                                            div { class: "votes-counter votes-counter--for",
                                                div { class: "votes-counter__line" }
                                                p { class: "votes-counter__title",
                                                    {translate!(i18, "governance.description.voting.for")}
                                                }
                                                p { class: "votes-counter__percent",
                                                    "50%"
                                                }
                                                p { class: "votes-counter__total",
                                                    "0 " {translate!(i18, "governance.description.voting.votes")}
                                                }
                                            }

                                            div { class: "votes-counter votes-counter--against",
                                                div { class: "votes-counter__line" }
                                                p { class: "votes-counter__title",
                                                    {translate!(i18, "governance.description.voting.against")}
                                                }
                                                p { class: "votes-counter__percent",
                                                    "50%"
                                                }
                                                p { class: "votes-counter__total",
                                                    "0 " {translate!(i18, "governance.description.voting.votes")}
                                                }
                                            }
                                        }

                                        hr { class: "form__divider" }

                                        div { class: "statistics__card",
                                            div { class: "",
                                                KeyValue {
                                                    class: "key-value--row",
                                                    size: ElementSize::Small,
                                                    text: translate!(i18, "governance.description.voting.threshold.title"),
                                                    body: rsx!(
                                                        "50% / 50%"
                                                    )
                                                }

                                                p { class: "vote-card__info",
                                                    {translate!(i18, "governance.description.voting.threshold.description")}
                                                }
                                            }

                                            KeyValue {
                                                class: "key-value--row",
                                                size: ElementSize::Small,
                                                text: translate!(i18, "governance.description.voting.total.title"),
                                                body: rsx!(
                                                    "0 " {translate!(i18, "governance.description.voting.total.voters")}
                                                )
                                            }
                                        }
                                    }
                                }
                                div { class: "vote-card",
                                    div { class: "row",
                                        Button {
                                            class: "",
                                            text: translate!(i18, "governance.description.voting.cta.for"),
                                            size: ElementSize::Small,
                                            variant: Variant::Secondary,
                                            on_click: move |_| {

                                            },
                                            status: None,
                                        }
                                        Button {
                                            class: "",
                                            text: translate!(i18, "governance.description.voting.cta.against"),
                                            size: ElementSize::Small,
                                            on_click: move |_| {

                                            },
                                            status: None,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "form__cta form__cta--initiatives",
                p { class: "wip",
                    {translate!(i18, "initiative.disclaimer")}
                }
            }
        }
    }
}
