use dioxus::prelude::*;

mod logos_page;
mod regular_page;
mod solid_page;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        h1 { "Bootstrap icons for Dioxus" }

        h2 { "Logos icons" }
        logos_page::LogosPage {}

        h2 { "Regular icons" }
        regular_page::RegularPage {}

        h2 { "solid icons" }
        solid_page::SolidPage {}
    }
}
