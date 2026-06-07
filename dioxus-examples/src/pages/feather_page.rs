use dioxus::prelude::*;

use dioxus_icons_feather::{Activity, AlertCircle, Archive, Bell, Book, Camera, Cloud, Heart, Home, Icon, Star};

/// Feather Icons page
#[component]
pub fn FeatherPage() -> Element {
    rsx! {
        div {
            id: "feather-icons",

            h1 { "Feather icons for Dioxus." }

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
                        icon: Activity,
                    }
                    span { "Activity" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: AlertCircle,
                    }
                    span { "AlertCircle" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Archive,
                    }
                    span { "Archive" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Bell,
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
                        icon: Book,
                    }
                    span { "Book" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Camera,
                    }
                    span { "Camera" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Cloud,
                    }
                    span { "Cloud" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Heart,
                    }
                    span { "Heart" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Home,
                    }
                    span { "Home" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Star,
                    }
                    span { "Star" }
                }
            }
        }
    }
}
