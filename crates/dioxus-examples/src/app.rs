use dioxus::prelude::*;

use crate::pages::ant_page::AntPage;
use crate::pages::bootstrap_page::BootstrapPage;
use crate::pages::box_page::BoxPage;
use crate::pages::circum_page::CircumPage;
use crate::pages::dev_page::DevPage;
use crate::pages::fa_page::FaPage;
use crate::pages::feather_page::FeatherPage;
use crate::pages::home_page::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/dioxus-icons-ant")]
    AntPage {},
    #[route("/dioxus-icons-box")]
    BoxPage {},
    #[route("/dioxus-icons-circum")]
    CircumPage {},
    #[route("/dioxus-icons-bootstrap")]
    BootstrapPage {},
    #[route("/dioxus-icons-dev")]
    DevPage {},
    #[route("/dioxus-icons-fa")]
    FaPage {},
    #[route("/dioxus-icons-feather")]
    FeatherPage {},
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
                to: Route::BoxPage {},
                "Box Icons"
            }
            Link {
                to: Route::BootstrapPage {},
                "Bootstrap Icons"
            }
            Link {
                to: Route::CircumPage {},
                "Circum Icons"
            }
            Link {
                to: Route::DevPage {},
                "Dev Icons"
            }
            Link {
                to: Route::FaPage {},
                "FA Icons"
            }
            Link {
                to: Route::FeatherPage {},
                "Feather Icons"
            }
        }

        Outlet::<Route> {}
    }
}
