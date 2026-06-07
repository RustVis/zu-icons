use dioxus::prelude::*;

mod filled_page;
mod outlined_page;
mod twotone_page;

use filled_page::FilledPage;
use outlined_page::OutlinedPage;
use twotone_page::TwotonePage;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        h1 { "Bootstrap icons for Dioxus" }

        h2 { "Filled icons" }
        FilledPage {}

        h2 { "Outlined icons" }
        OutlinedPage {}

        h2 { "Twotone icons" }
        TwotonePage {}
    }
}
