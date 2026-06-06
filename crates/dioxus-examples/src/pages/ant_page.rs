use dioxus::prelude::*;

use dioxus_icons_ant::Icon;
use dioxus_icons_ant::filled::{AccountBook, Alert, AlipayCircle, AlipaySquare, Aliwangwang};

/// Ant Icons page
#[component]
pub fn AntPage() -> Element {
    rsx! {
        div {
            id: "ant-icons",
            h1 { "Dioxus Icons - Ant Design" }
            p { "Ant Design icons for Dioxus." }

            div {
                display: "flex",
                flex_direction: "row",
                gap: "24px",
                flex_wrap: "wrap",

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: AccountBook,
                    }
                    span { "AccountBook" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Alert,
                    }
                    span { "Alert" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: AlipayCircle,
                    }
                    span { "AlipayCircle" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: AlipaySquare,
                    }
                    span { "AlipaySquare" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Aliwangwang,
                    }
                    span { "Aliwangwang" }
                }
            }
        }
    }
}
