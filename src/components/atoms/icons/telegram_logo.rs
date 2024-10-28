use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct TelegramLogo;
impl IconShape for TelegramLogo {
    fn view_box(&self) -> String {
        String::from("0 0 20 21")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path {
                d: "M17.2077 3.67849L2.44425 9.10906C1.4367 9.49509 1.44253 10.0312 2.25939 10.2703L6.04976 11.3982L14.8196 6.12014C15.2343 5.87947 15.6131 6.00894 15.3017 6.27265L8.19642 12.3895H8.19475L8.19642 12.3903L7.93496 16.1171C8.31799 16.1171 8.48702 15.9495 8.70186 15.7517L10.5429 14.044L14.3724 16.7422C15.0785 17.1131 15.5856 16.9225 15.7613 16.1187L18.2752 4.8175C18.5325 3.83338 17.8814 3.38778 17.2077 3.67849V3.67849Z"
            }
        )
    }
}
