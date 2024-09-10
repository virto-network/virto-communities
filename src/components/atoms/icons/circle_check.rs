use dioxus::prelude::*;
use super::icon::IconShape;
#[derive(PartialEq, Clone)]
pub struct CircleCheck;
impl IconShape for CircleCheck {
    fn view_box(&self) -> String {
        String::from("0 0 36 36")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M18 33C26.2843 33 33 26.2843 33 18C33 9.71573 26.2843 3 18 3C9.71573 3 3 9.71573 3 18C3 26.2843 9.71573 33 18 33ZM25.5001 15.6213C26.0859 15.0355 26.0859 14.0858 25.5001 13.5C24.9144 12.9142 23.9646 12.9142 23.3788 13.5L15.5242 21.3546L12.0784 17.3345C11.5392 16.7055 10.5923 16.6326 9.96329 17.1718C9.3343 17.7109 9.26146 18.6578 9.8006 19.2868L14.3006 24.5368C14.5725 24.8541 14.9644 25.0435 15.3819 25.0595C15.7994 25.0756 16.2047 24.9167 16.5001 24.6213L25.5001 15.6213Z"
            }
        )
    }
}
