use dioxus::prelude::*;

mod outline_page;
mod solid_page;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        h1 { "Bootstrap icons for Dioxus" }

        h2 { "Outline icons" }
        outline_page::OutlinePage {}

        h2 { "Solid icons" }
        solid_page::SolidPage {}
    }
}
