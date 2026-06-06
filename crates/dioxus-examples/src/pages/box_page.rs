use dioxus::prelude::*;

use dioxus_icons_box::{Icon, logos, regular, solid};

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
                        icon: regular::BxAlarm,
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
                        icon: regular::BxAlbum,
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
                        icon: regular::BxAnchor,
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
                        icon: regular::BxArchive,
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
                        icon: regular::BxCalendar,
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
                        icon: solid::BxsAlarm,
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
                        icon: solid::BxsAlbum,
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
                        icon: solid::BxsAmbulance,
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
                        icon: solid::BxsArchive,
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
                        icon: solid::BxsCalendar,
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
                        icon: logos::BxlAdobe,
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
                        icon: logos::BxlAirbnb,
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
                        icon: logos::BxlAmazon,
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
                        icon: logos::BxlAndroid,
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
                        icon: logos::BxlApple,
                    }
                    span { "BxlApple" }
                }
            }
        }
    }
}
