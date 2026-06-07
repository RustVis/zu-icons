use dioxus::prelude::*;

use dioxus_icons_line_awesome::{
    AddressBook, ArchiveSolid, Bell, BookSolid, Bookmark, Calendar, CameraSolid, CloudSolid,
    Heart, HomeSolid, Icon, Star,
};

/// Line Awesome Icons page
#[component]
pub fn LineAwesomePage() -> Element {
    rsx! {
        div {
            id: "line-awesome-icons",

            h1 { "Line Awesome icons for Dioxus." }

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
                        icon: AddressBook,
                    }
                    span { "AddressBook" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: ArchiveSolid,
                    }
                    span { "ArchiveSolid" }
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
                        icon: BookSolid,
                    }
                    span { "BookSolid" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Bookmark,
                    }
                    span { "Bookmark" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: Calendar,
                    }
                    span { "Calendar" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: CameraSolid,
                    }
                    span { "CameraSolid" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: CloudSolid,
                    }
                    span { "CloudSolid" }
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
                        icon: HomeSolid,
                    }
                    span { "HomeSolid" }
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
