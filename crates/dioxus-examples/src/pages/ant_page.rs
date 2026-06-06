use dioxus::prelude::*;

use dioxus_icons_ant::filled::{AccountBook, Alert, AlipayCircle, AlipaySquare, Aliwangwang};
use dioxus_icons_ant::{Icon, IconShape};

/// Ant Icons page
#[component]
pub fn AntPage() -> Element {
    rsx! {
        div {
            id: "ant-icons",
            h1 { "Dioxus Icons - Ant Design" }
            p { "Ant Design icons for Dioxus." }
            Icon {
                icon: AccountBook,
            }

            Icon {
                icon: Alert,
            }

            Icon {
                icon: AlipayCircle,
            }

            Icon {
                icon: AlipaySquare,
            }

            button {
                label {"Aliwangwang"}
                Icon {
                    icon: Aliwangwang,
                }
            }
        }
    }
}
