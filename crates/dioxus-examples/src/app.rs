use dioxus::prelude::*;

use crate::pages::ant_page::AntPage;
use crate::pages::circum_page::CircumPage;
use crate::pages::home_page::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/dioxus-icons-ant")]
    AntPage {},
    #[route("/dioxus-icons-circum")]
    CircumPage {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::AntPage {},
                "Ant Icons"
            }
            Link {
                to: Route::CircumPage {},
                "Circum Icons"
            }
        }

        Outlet::<Route> {}
    }
}
