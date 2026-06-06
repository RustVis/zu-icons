use dioxus::prelude::*;

use dioxus_icons_md::{filled, outlined, round, sharp, twotone, Icon};

/// Material Design Icons page
#[component]
pub fn MaterialPage() -> Element {
    rsx! {
        div {
            id: "material-icons",

            h1 { "Material Design icons for Dioxus." }

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
                        icon: filled::Alarm,
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
                        icon: filled::Archive,
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
                        icon: filled::Book,
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
                        icon: filled::CalendarMonth,
                    }
                    span { "CalendarMonth" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: filled::Camera,
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
                        icon: filled::Cloud,
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
                        icon: filled::Favorite,
                    }
                    span { "Favorite" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: filled::Home,
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
                        icon: filled::Star,
                    }
                    span { "Star" }
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
                        icon: outlined::Alarm,
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
                        icon: outlined::Archive,
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
                        icon: outlined::Book,
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
                        icon: outlined::CalendarMonth,
                    }
                    span { "CalendarMonth" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: outlined::Camera,
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
                        icon: outlined::Cloud,
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
                        icon: outlined::Favorite,
                    }
                    span { "Favorite" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: outlined::Home,
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
                        icon: outlined::Star,
                    }
                    span { "Star" }
                }
            }

            h2 { "Round" }

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
                        icon: round::Alarm,
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
                        icon: round::Archive,
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
                        icon: round::Book,
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
                        icon: round::CalendarMonth,
                    }
                    span { "CalendarMonth" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: round::Camera,
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
                        icon: round::Cloud,
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
                        icon: round::Favorite,
                    }
                    span { "Favorite" }
                }

                div {
                    display: "flex",
                    flex_direction: "column",
                    align_items: "center",
                    Icon {
                        width: "32",
                        height: "32",
                        icon: round::Home,
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
                        icon: round::Star,
                        }
                        span { "Star" }
                    }
                }

                h2 { "Sharp" }

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
                            icon: sharp::Alarm,
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
                            icon: sharp::Archive,
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
                            icon: sharp::Book,
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
                            icon: sharp::CalendarMonth,
                        }
                        span { "CalendarMonth" }
                    }

                    div {
                        display: "flex",
                        flex_direction: "column",
                        align_items: "center",
                        Icon {
                            width: "32",
                            height: "32",
                            icon: sharp::Camera,
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
                            icon: sharp::Cloud,
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
                            icon: sharp::Favorite,
                        }
                        span { "Favorite" }
                    }

                    div {
                        display: "flex",
                        flex_direction: "column",
                        align_items: "center",
                        Icon {
                            width: "32",
                            height: "32",
                            icon: sharp::Home,
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
                            icon: sharp::Star,
                        }
                        span { "Star" }
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
                            icon: twotone::Alarm,
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
                            icon: twotone::Archive,
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
                            icon: twotone::Book,
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
                            icon: twotone::CalendarMonth,
                        }
                        span { "CalendarMonth" }
                    }

                    div {
                        display: "flex",
                        flex_direction: "column",
                        align_items: "center",
                        Icon {
                            width: "32",
                            height: "32",
                            icon: twotone::Camera,
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
                            icon: twotone::Cloud,
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
                            icon: twotone::Favorite,
                        }
                        span { "Favorite" }
                    }

                    div {
                        display: "flex",
                        flex_direction: "column",
                        align_items: "center",
                        Icon {
                            width: "32",
                            height: "32",
                            icon: twotone::Home,
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
                            icon: twotone::Star,
                        }
                        span { "Star" }
                    }
                }
            }
        }
    }
