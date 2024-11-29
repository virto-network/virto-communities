use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct GlobalLine;
impl IconShape for GlobalLine {
    fn view_box(&self) -> String {
        String::from("0 0 56 57")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path {
                d: "M28.0001 51.36C15.1131 51.36 4.66675 40.9137 4.66675 28.0267C4.66675 15.1397 15.1131 4.69336 28.0001 4.69336C40.8871 4.69336 51.3334 15.1397 51.3334 28.0267C51.3334 40.9137 40.8871 51.36 28.0001 51.36ZM22.6567 45.9164C20.3547 41.0334 19.0211 35.7505 18.7297 30.36H9.47808C9.93212 33.9508 11.4192 37.3324 13.7587 40.094C16.0983 42.8556 19.1894 44.8783 22.6567 45.9164ZM23.4034 30.36C23.7557 36.051 25.3821 41.3967 28.0001 46.1147C30.6888 41.272 32.2595 35.8888 32.5968 30.36H23.4034ZM46.5221 30.36H37.2704C36.979 35.7505 35.6454 41.0334 33.3434 45.9164C36.8107 44.8783 39.9019 42.8556 42.2414 40.094C44.5809 37.3324 46.0681 33.9508 46.5221 30.36ZM9.47808 25.6934H18.7297C19.0211 20.3029 20.3547 15.02 22.6567 10.137C19.1894 11.1751 16.0983 13.1978 13.7587 15.9594C11.4192 18.721 9.93212 22.1026 9.47808 25.6934ZM23.4058 25.6934H32.5944C32.2578 20.1648 30.688 14.7816 28.0001 9.93869C25.3113 14.7814 23.7407 20.1646 23.4034 25.6934H23.4058ZM33.3434 10.137C35.6454 15.02 36.979 20.3029 37.2704 25.6934H46.5221C46.0681 22.1026 44.5809 18.721 42.2414 15.9594C39.9019 13.1978 36.8107 11.1751 33.3434 10.137Z",
            }
        )
    }
}