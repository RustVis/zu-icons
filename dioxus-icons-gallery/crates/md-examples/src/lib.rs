use dioxus::prelude::*;

mod filled_page;
mod outlined_page;
mod round_page;
mod sharp_page;
mod twotone_page;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        h1 { "Bootstrap icons for Dioxus" }

        h2 { "Filled icons" }
        filled_page::FilledPage {}

        h2 { "Outlined icons" }
        outlined_page::OutlinedPage {}

        h2 { "Twotone icons" }
        twotone_page::TwotonePage {}

        h2 { "Round icons" }
        round_page::RoundPage {}

        h2 { "Sharp icons" }
        sharp_page::SharpPage {}
    }
}
