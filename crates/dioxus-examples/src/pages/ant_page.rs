use dioxus::prelude::*;

/// Ant Icons page
#[component]
pub fn AntPage() -> Element {
    rsx! {
        div {
            id: "ant-icons",
            h1 { "Dioxus Icons - Ant Design" }
            p { "Ant Design icons for Dioxus." }
        }
    }
}
