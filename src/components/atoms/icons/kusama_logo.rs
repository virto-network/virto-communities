use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct KusamaLogo;
impl IconShape for KusamaLogo {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path { d: "M22.5972 5.7833C22.2361 5.49858 21.8056 5.10969 21.0208 5.01247C20.2847 4.91525 19.5347 5.4083 19.0278 5.73469C18.5208 6.06108 17.5625 7.01941 17.1667 7.31108C16.7708 7.60275 15.7569 7.87358 14.125 8.85275C12.4931 9.83191 6.09028 13.943 6.09028 13.943L7.75694 13.9639L0.326389 17.7902H1.06944L0 18.6027C0 18.6027 0.944444 18.8527 1.73611 18.3527V18.5819C1.73611 18.5819 10.5833 15.0958 12.2917 15.9986L11.25 16.3041C11.3403 16.3041 13.0208 16.4152 13.0208 16.4152C13.0616 16.7633 13.1786 17.098 13.3634 17.3957C13.5483 17.6933 13.7964 17.9466 14.0903 18.1375C15.1042 18.8041 15.125 19.1722 15.125 19.1722C15.125 19.1722 14.5972 19.3875 14.5972 19.6583C14.5972 19.6583 15.375 19.4222 16.0972 19.443C16.5556 19.4604 17.0102 19.5327 17.4514 19.6583C17.4514 19.6583 17.3958 19.3666 16.6944 19.1722C15.9931 18.9777 15.2986 18.2139 14.9583 17.7972C14.7499 17.532 14.6141 17.2171 14.5641 16.8835C14.5142 16.5499 14.5519 16.209 14.6736 15.8944C14.9167 15.2625 15.7639 14.9152 17.5139 14.0125C19.5764 12.943 20.0486 12.1514 20.3403 11.5333C20.6319 10.9152 21.0625 9.68608 21.3056 9.10969C21.6111 8.36664 21.9861 7.9708 22.2986 7.73469C22.6111 7.49858 24 6.97775 24 6.97775C24 6.97775 22.9375 6.05414 22.5972 5.7833Z" }
        )
    }
}
