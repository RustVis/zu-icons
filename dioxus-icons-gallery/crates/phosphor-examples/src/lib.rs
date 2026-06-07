use dioxus::prelude::*;

mod bold_page;
mod duotone_page;
mod fill_page;
mod light_page;
mod regular_page;
mod thin_page;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        h1 { "Bootstrap icons for Dioxus" }

        h2 { "Bold icons" }
        bold_page::BoldPage {}

        h2 { "Duotone icons" }
        duotone_page::DuotonePage {}

        h2 { "Fill icons" }
        fill_page::FillPage {}

        h2 { "Light icons" }
        light_page::LightPage {}

        h2 { "Regular icons" }
        regular_page::RegularPage {}

        h2 { "Thin icons" }
        thin_page::ThinPage {}
    }
}
