use dioxus::prelude::*;

use dioxus_icons_box::Icon;
use dioxus_icons_box::logos::{BxlAdobe, BxlAirbnb, BxlAmazon, BxlAndroid, BxlApple};
use dioxus_icons_box::regular::{BxAlarm, BxAlbum, BxAnchor, BxArchive, BxCalendar};
use dioxus_icons_box::solid::{BxsAlarm, BxsAlbum, BxsAmbulance, BxsArchive, BxsCalendar};

/// Box Icons page
#[component]
pub fn BoxPage() -> Element {
    rsx! {
        div {
            id: "box-icons",

            h1 { "Box Icons for Dioxus." }

            h2 { "Regular" }

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
                        icon: BxAlarm,
                    }
                    span { "BxAlarm" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BxAlbum,
                    }
                    span { "BxAlbum" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BxAnchor,
                    }
                    span { "BxAnchor" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BxArchive,
                    }
                    span { "BxArchive" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BxCalendar,
                    }
                    span { "BxCalendar" }
                }
            }

            h2 { "Solid" }

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
                        icon: BxsAlarm,
                    }
                    span { "BxsAlarm" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BxsAlbum,
                    }
                    span { "BxsAlbum" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BxsAmbulance,
                    }
                    span { "BxsAmbulance" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BxsArchive,
                    }
                    span { "BxsArchive" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BxsCalendar,
                    }
                    span { "BxsCalendar" }
                }
            }

            h2 { "Logos" }

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
                        icon: BxlAdobe,
                    }
                    span { "BxlAdobe" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BxlAirbnb,
                    }
                    span { "BxlAirbnb" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BxlAmazon,
                    }
                    span { "BxlAmazon" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BxlAndroid,
                    }
                    span { "BxlAndroid" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: BxlApple,
                    }
                    span { "BxlApple" }
                }
            }
        }
    }
}
