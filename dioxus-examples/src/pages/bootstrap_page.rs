use dioxus::prelude::*;

use dioxus_icons_bs::Icon;
use dioxus_icons_bs::{
    Activity, Airplane, Alarm, Alexa, Alipay, Amazon, Android, Apple, Archive, ArrowClockwise,
};

/// Bootstrap Icons page
#[component]
pub fn BootstrapPage() -> Element {
    rsx! {
        div {
            id: "bootstrap-icons",
            h1 { "Bootstrap icons for Dioxus" }

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
                        icon: Airplane,
                    }
                    span { "Airplane" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Alarm,
                    }
                    span { "Alarm" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Alexa,
                    }
                    span { "Alexa" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Alipay,
                    }
                    span { "Alipay" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Amazon,
                    }
                    span { "Amazon" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Android,
                    }
                    span { "Android" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Apple,
                    }
                    span { "Apple" }
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
                        icon: ArrowClockwise,
                    }
                    span { "ArrowClockwise" }
                }
            }
        }
    }
}
