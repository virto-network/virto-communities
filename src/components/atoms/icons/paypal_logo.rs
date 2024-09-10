use dioxus::prelude::*;

use super::icon::IconShape;

#[derive(PartialEq, Clone)]
pub struct PaypalLogo;
impl IconShape for PaypalLogo {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }
    fn child_elements(&self) -> Element {
        rsx!(
            path {
                d: "M20.9746 9.01583C21.1183 9.68468 21.1005 10.4961 20.927 11.4524C20.2356 14.9903 17.9844 16.758 14.1756 16.758H13.6505C13.4565 16.7569 13.2688 16.827 13.1231 16.9552C12.9721 17.086 12.8717 17.2655 12.8391 17.4625L12.7904 17.687L12.1335 21.8201L12.1085 21.9995C12.0746 22.1979 11.9705 22.3776 11.8151 22.5056C11.6685 22.6346 11.4793 22.7048 11.284 22.7028H8.28672C8.2117 22.7067 8.13683 22.6926 8.0684 22.6616C7.99997 22.6306 7.93999 22.5836 7.89349 22.5246C7.84627 22.4643 7.81233 22.3947 7.7939 22.3204C7.77547 22.2461 7.77298 22.1687 7.78657 22.0933C7.85904 21.6502 7.9624 20.979 8.10377 20.0868C8.24277 19.1958 8.3485 18.5258 8.42097 18.0803C8.49343 17.6348 8.59917 16.9659 8.7441 16.0785C8.88785 15.1899 8.99596 14.5222 9.06605 14.0767C9.10525 13.7821 9.2787 13.636 9.58045 13.636H11.1439C12.2047 13.6514 13.1421 13.5683 13.9654 13.3853C15.3577 13.074 16.5006 12.5014 17.3939 11.6639C18.2077 10.9071 18.8231 9.92703 19.2472 8.72596C19.4397 8.1676 19.5763 7.63657 19.6654 7.13642C19.6725 7.08771 19.682 7.05801 19.6951 7.04851C19.7046 7.03544 19.7212 7.03188 19.7367 7.03544C19.7625 7.04695 19.7871 7.06087 19.8103 7.07702C20.4328 7.54984 20.8249 8.19493 20.9746 9.01583ZM18.9217 5.64667C18.9217 6.49846 18.7387 7.43817 18.3693 8.46698C17.7313 10.3226 16.5314 11.5772 14.7613 12.2306C13.8608 12.5501 12.8581 12.7153 11.7497 12.7355C11.7497 12.7426 11.3922 12.7438 10.6758 12.7438L9.60303 12.7355C8.80469 12.7355 8.33543 13.1156 8.19287 13.8807C8.17743 13.9437 7.83885 16.0547 7.17713 20.2115C7.16763 20.29 7.12011 20.3327 7.03338 20.3327H3.51096C3.42795 20.3345 3.34558 20.3178 3.26978 20.2839C3.19398 20.25 3.12664 20.1997 3.07259 20.1367C3.01651 20.0747 2.97511 20.0008 2.95146 19.9206C2.92781 19.8404 2.92252 19.7559 2.93597 19.6734L5.70639 2.09692C5.74187 1.87201 5.85851 1.66788 6.03427 1.52312C6.20564 1.37562 6.42476 1.29541 6.65085 1.2974H13.7955C14.0663 1.2974 14.4548 1.34967 14.9585 1.45303C15.467 1.55282 15.9101 1.6835 16.2927 1.83438C17.1456 2.15989 17.7967 2.65172 18.2469 3.30393C18.6972 3.95971 18.9217 4.73785 18.9217 5.64667Z"
            }
        )
    }
}
