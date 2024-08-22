use dioxus::prelude::*;
use super::icon::IconShape;
#[derive(PartialEq, Clone)]
pub struct Glyph;
impl IconShape for Glyph {
    fn view_box(&self) -> String {
        String::from("0 0 16 15")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            g { filter: "url(#filter0_d_575_469)",
                path {
                    fill_rule: "evenodd",
                    clip_rule: "evenodd",
                    d: "M13.3851 0.996566C13.9392 0.380801 14.8877 0.330883 15.5034 0.885072C16.1192 1.43926 16.1691 2.38769 15.6149 3.00346L6.61494 13.0035C6.04003 13.6422 5.04703 13.6684 4.43934 13.0607L0.43934 9.06067C-0.146447 8.47489 -0.146447 7.52514 0.43934 6.93935C1.02513 6.35357 1.97487 6.35357 2.56066 6.93935L5.44271 9.8214L13.3851 0.996566Z"
                }
            }
        )
    }
}
