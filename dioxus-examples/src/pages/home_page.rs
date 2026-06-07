use dioxus::prelude::*;

const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {

    p { dir: "auto",
        a {
            href: "https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg",
            rel: "noopener noreferrer",
            target: "_blank",
            img {
                alt: "Build status",
                src: "https://github.com/RustVis/zu-icons/actions/workflows/ci.yml/badge.svg",
                style: "max-width: 100%;",
            }
        }
        a {
            href: "https://camo.githubusercontent.com/1d3e7ddf31204702b31748701fe12a0242d9354a03da2bb0c4f5f994028549e7/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f72757374632d312e38382b2d677265656e2e737667",
            rel: "noopener noreferrer nofollow",
            target: "_blank",
            img {
                alt: "Minimum rustc version",
                "data-canonical-src": "https://img.shields.io/badge/rustc-1.88+-green.svg",
                src: "https://camo.githubusercontent.com/1d3e7ddf31204702b31748701fe12a0242d9354a03da2bb0c4f5f994028549e7/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f72757374632d312e38382b2d677265656e2e737667",
                style: "max-width: 100%;",
            }
        }
        a {
            href: "https://camo.githubusercontent.com/da6deab0dfc544bee363a4a3c32733ee73e1db1b2ff08373a2ea04a033a5048c/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f6c2f64696f7875732d69636f6e2d636f6d706f6e656e742e737667",
            rel: "noopener noreferrer nofollow",
            target: "_blank",
            img {
                alt: "License",
                "data-canonical-src": "https://img.shields.io/crates/l/dioxus-icon-component.svg",
                src: "https://camo.githubusercontent.com/da6deab0dfc544bee363a4a3c32733ee73e1db1b2ff08373a2ea04a033a5048c/68747470733a2f2f696d672e736869656c64732e696f2f6372617465732f6c2f64696f7875732d69636f6e2d636f6d706f6e656e742e737667",
                style: "max-width: 100%;",
            }
        }
    }
    p { dir: "auto",
        "Open source icons for the "
        a { href: "https://dioxuslabs.com/", rel: "nofollow", "Dioxus" }
        " framework."
    }
    }
}

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
    }
}
