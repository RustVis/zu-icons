use dioxus::prelude::*;
use dioxus_icons_circum::{AlarmOff, Icon};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }


            div {
                p {
                    "Icons"
                }

                p {
                    Icon {
                        width: "1em",
                        height: "1em",
                        fill: "none",
                        title: "Alarm",
                        icon: AlarmOff {}
                    }
                }
            }
        }
    }
}
