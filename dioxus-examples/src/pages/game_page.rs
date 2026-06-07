use dioxus::prelude::*;

use dioxus_icons_game::{darkzaitzev, delapouite, lorc, Icon};

/// Game Icons page
#[component]
pub fn GamePage() -> Element {
    rsx! {
        div {
            id: "game-icons",

            h1 { "Game icons for Dioxus." }

            h2 { "Lorc" }

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
                        icon: lorc::AcidBlob,
                    }
                    span { "AcidBlob" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: lorc::Acorn,
                    }
                    span { "Acorn" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: lorc::AerialSignal,
                    }
                    span { "AerialSignal" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: lorc::Aerosol,
                    }
                    span { "Aerosol" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: lorc::Afterburn,
                    }
                    span { "Afterburn" }
                }
            }

            h2 { "Delapouite" }

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
                        icon: delapouite::Abacus,
                    }
                    span { "Abacus" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: delapouite::AbbotMeeple,
                    }
                    span { "AbbotMeeple" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: delapouite::AchillesHeel,
                    }
                    span { "AchillesHeel" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: delapouite::AeroBike,
                    }
                    span { "AeroBike" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: delapouite::Africa,
                    }
                    span { "Africa" }
                }
            }

            h2 { "Dark Zaitzev" }

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
                        icon: darkzaitzev::Acrobatic,
                    }
                    span { "Acrobatic" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: darkzaitzev::Apothecary,
                    }
                    span { "Apothecary" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: darkzaitzev::BigGear,
                    }
                    span { "BigGear" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: darkzaitzev::Catch,
                    }
                    span { "Catch" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: darkzaitzev::Cauldron,
                    }
                    span { "Cauldron" }
                }
            }
        }
    }
}
