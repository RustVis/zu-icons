use dioxus::prelude::*;

use dioxus_icons_circum::Icon;
use dioxus_icons_circum::{
    AirportSign1, AlarmOff, AlarmOn, Apple, At, Avocado, Bacon, BadgeDollar, Bag1, Bank,
};

/// Circum Icons page
#[component]
pub fn CircumPage() -> Element {
    rsx! {
        div {
            id: "circum-icons",
            h1 { "Circum icons for Dioxus" }

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
                        icon: AirportSign1,
                    }
                    span { "AirportSign1" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: AlarmOff,
                    }
                    span { "AlarmOff" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: AlarmOn,
                    }
                    span { "AlarmOn" }
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
                        icon: At,
                    }
                    span { "At" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Avocado,
                    }
                    span { "Avocado" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Bacon,
                    }
                    span { "Bacon" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BadgeDollar,
                    }
                    span { "BadgeDollar" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Bag1,
                        }
                        span { "Bag1" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Bank,
                        }
                        span { "Bank" }
                }
            }
        }
    }
}
