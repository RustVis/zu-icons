use dioxus::prelude::*;

use dioxus_icons_ant::filled;
use dioxus_icons_ant::outlined;
use dioxus_icons_ant::{Icon, twotone};

/// Ant Icons page
#[component]
pub fn AntPage() -> Element {
    rsx! {
        div {
            id: "ant-icons",

            h1 { "Ant Design icons for Dioxus." }

            h2 { "Filled" }

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
                        icon: filled::AccountBook,
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
                        icon: filled::Alert,
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
                        icon: filled::AlipayCircle,
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
                        icon: filled::AlipaySquare,
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
                        icon: filled::Aliwangwang,
                    }
                    span { "Aliwangwang" }
                }
            }

            h2 { "Outlined" }

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
                        icon: outlined::AccountBook,
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
                        icon: outlined::Alert,
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
                        icon: outlined::Appstore,
                    }
                    span { "Appstore" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: outlined::Bell,
                    }
                    span { "Bell" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: outlined::Book,
                    }
                    span { "Book" }
                }
            }

            h2 { "Twotone" }

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
                        icon: twotone::AccountBook,
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
                        icon: twotone::Alert,
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
                        icon: twotone::Api,
                    }
                    span { "Api" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: twotone::Appstore,
                    }
                    span { "Appstore" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: twotone::Audio,
                    }
                    span { "Audio" }
                }
            }
        }
    }
}
