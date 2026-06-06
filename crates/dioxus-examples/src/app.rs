use dioxus::prelude::*;

use crate::pages::ant_page::AntPage;
use crate::pages::bootstrap_page::BootstrapPage;
use crate::pages::box_page::BoxPage;
use crate::pages::circum_page::CircumPage;
use crate::pages::dev_page::DevPage;
use crate::pages::fa_page::FaPage;
use crate::pages::feather_page::FeatherPage;
use crate::pages::game_page::GamePage;
use crate::pages::grommet_page::GrommetPage;
use crate::pages::hero_page::HeroPage;
use crate::pages::home_page::Home;
use crate::pages::ionic_page::IonicPage;
use crate::pages::line_awesome_page::LineAwesomePage;
use crate::pages::lucide_page::LucidePage;
use crate::pages::material_page::MaterialPage;
use crate::pages::octicon_page::OcticonPage;
use crate::pages::phosphor_page::PhosphorPage;
use crate::pages::radix_page::RadixPage;
use crate::pages::remix_page::RemixPage;

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
    #[route("/dioxus-icons-game")]
    GamePage {},
    #[route("/dioxus-icons-grommet")]
    GrommetPage {},
    #[route("/dioxus-icons-hero")]
    HeroPage {},
    #[route("/dioxus-icons-ionic")]
    IonicPage {},
    #[route("/dioxus-icons-line-awesome")]
    LineAwesomePage {},
    #[route("/dioxus-icons-lucide")]
    LucidePage {},
    #[route("/dioxus-icons-md")]
    MaterialPage {},
    #[route("/dioxus-icons-oct")]
    OcticonPage {},
    #[route("/dioxus-icons-phosphor")]
    PhosphorPage {},
    #[route("/dioxus-icons-radix")]
    RadixPage {},
    #[route("/dioxus-icons-remix")]
    RemixPage {},
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
            Link {
                to: Route::GamePage {},
                "Game Icons"
            }
            Link {
                to: Route::GrommetPage {},
                "Grommet Icons"
            }
            Link {
                to: Route::HeroPage {},
                "Hero Icons"
            }
            Link {
                to: Route::IonicPage {},
                "Ionic Icons"
            }
            Link {
                to: Route::LineAwesomePage {},
                "Line Awesome Icons"
            }
            Link {
                to: Route::LucidePage {},
                "Lucide Icons"
            }
            Link {
                to: Route::MaterialPage {},
                "Material Icons"
            }
            Link {
                to: Route::OcticonPage {},
                "Octicon Icons"
            }
            Link {
                to: Route::PhosphorPage {},
                "Phosphor Icons"
            }
            Link {
                to: Route::RadixPage {},
                "Radix Icons"
            }
            Link {
                to: Route::RemixPage {},
                "Remix Icons"
            }
        }

        Outlet::<Route> {}
    }
}
