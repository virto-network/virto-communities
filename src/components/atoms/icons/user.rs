use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct User;
impl IconShape for User {
    fn view_box(&self) -> String {
        String::from("0 0 32 32")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M10.1801 8.38091C10.1801 5.225 12.7854 2.66663 15.9991 2.66663C19.2128 2.66663 21.8181 5.225 21.8181 8.38091C21.8181 11.5368 19.2128 14.0952 15.9991 14.0952C12.7854 14.0952 10.1801 11.5368 10.1801 8.38091Z" }
            path { d: "M5.78337 23.4478C7.10631 19.1384 11.1793 16 15.999 16C20.8187 16 24.8918 19.1385 26.2146 23.4481C26.7207 25.0968 25.5562 26.6579 23.8597 26.968C22.0521 27.2984 19.4018 27.626 15.999 27.626C12.596 27.626 9.9457 27.2983 8.13807 26.9677C6.44159 26.6576 5.27725 25.0965 5.78337 23.4478Z" }
        )
    }
}