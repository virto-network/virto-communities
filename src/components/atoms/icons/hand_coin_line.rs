use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct HandCoinLine;
impl IconShape for HandCoinLine {
    fn view_box(&self) -> String {
        String::from("0 0 25 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            g {
                clip_path: "url(#clip0_1543_16050)",
                path {
                    d: "M5.5 9C5.76522 9 6.01957 9.10536 6.20711 9.29289C6.39464 9.48043 6.5 9.73478 6.5 10C8.07114 9.99769 9.59698 10.5263 10.83 11.5H13C14.333 11.5 15.53 12.08 16.354 13H19.5C20.4453 12.9997 21.3712 13.2674 22.1705 13.772C22.9698 14.2767 23.6097 14.9975 24.016 15.851C21.651 18.972 17.822 21 13.5 21C10.71 21 8.35 20.397 6.44 19.342C6.37004 19.5351 6.24224 19.7018 6.07402 19.8196C5.90579 19.9374 5.70534 20.0003 5.5 20H2.5C2.23478 20 1.98043 19.8946 1.79289 19.7071C1.60536 19.5196 1.5 19.2652 1.5 19V10C1.5 9.73478 1.60536 9.48043 1.79289 9.29289C1.98043 9.10536 2.23478 9 2.5 9H5.5ZM6.501 12L6.5 17.022L6.545 17.054C8.34 18.314 10.678 19 13.5 19C16.504 19 19.299 17.844 21.335 15.87L21.468 15.737L21.348 15.637C20.8758 15.2672 20.3034 15.0477 19.705 15.007L19.5 15H17.389C17.461 15.322 17.5 15.656 17.5 16V17H8.5V15L15.29 14.999L15.256 14.921C15.0644 14.5205 14.7696 14.1783 14.4019 13.9295C14.0343 13.6806 13.607 13.5341 13.164 13.505L13 13.5H10.07C9.60531 13.0247 9.05027 12.6472 8.43752 12.3896C7.82477 12.132 7.16669 11.9995 6.502 12H6.501ZM4.5 11H3.5V18H4.5V11ZM18.5 5C19.2956 5 20.0587 5.31607 20.6213 5.87868C21.1839 6.44129 21.5 7.20435 21.5 8C21.5 8.79565 21.1839 9.55871 20.6213 10.1213C20.0587 10.6839 19.2956 11 18.5 11C17.7044 11 16.9413 10.6839 16.3787 10.1213C15.8161 9.55871 15.5 8.79565 15.5 8C15.5 7.20435 15.8161 6.44129 16.3787 5.87868C16.9413 5.31607 17.7044 5 18.5 5ZM18.5 7C18.2348 7 17.9804 7.10536 17.7929 7.29289C17.6054 7.48043 17.5 7.73478 17.5 8C17.5 8.26522 17.6054 8.51957 17.7929 8.70711C17.9804 8.89464 18.2348 9 18.5 9C18.7652 9 19.0196 8.89464 19.2071 8.70711C19.3946 8.51957 19.5 8.26522 19.5 8C19.5 7.73478 19.3946 7.48043 19.2071 7.29289C19.0196 7.10536 18.7652 7 18.5 7ZM11.5 2C12.2956 2 13.0587 2.31607 13.6213 2.87868C14.1839 3.44129 14.5 4.20435 14.5 5C14.5 5.79565 14.1839 6.55871 13.6213 7.12132C13.0587 7.68393 12.2956 8 11.5 8C10.7044 8 9.94129 7.68393 9.37868 7.12132C8.81607 6.55871 8.5 5.79565 8.5 5C8.5 4.20435 8.81607 3.44129 9.37868 2.87868C9.94129 2.31607 10.7044 2 11.5 2ZM11.5 4C11.2348 4 10.9804 4.10536 10.7929 4.29289C10.6054 4.48043 10.5 4.73478 10.5 5C10.5 5.26522 10.6054 5.51957 10.7929 5.70711C10.9804 5.89464 11.2348 6 11.5 6C11.7652 6 12.0196 5.89464 12.2071 5.70711C12.3946 5.51957 12.5 5.26522 12.5 5C12.5 4.73478 12.3946 4.48043 12.2071 4.29289C12.0196 4.10536 11.7652 4 11.5 4Z",
                    fill: "#56C960"
                }
            }
            defs {
                clipPath {
                    id: "clip0_1543_16050",
                    rect {
                        width: "24",
                        height: "24",
                        fill: "white",
                        transform: "translate(0.5)"
                    }
                }
            }
        )
    }
}
