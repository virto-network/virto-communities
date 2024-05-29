use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct AccountProps {
    title: String,
    description: String,
    on_click: EventHandler<()>,
}

pub fn AccountButton(props: AccountProps) -> Element {
    rsx!(
       button {
           class: "account",
           onclick: move |_| {
              props.on_click.call(())
           },
           div {
               class: "account__wrapper",
               h3 {
                   class: "account__title",
                   {props.title}
               }
               p {
                   class: "account__description",
                   {props.description}
               }
           }
       }
    )
}
