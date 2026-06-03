// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TorBrowserProps {
    #[props(default = None)]
    pub title: Option<&'static str>,

    #[props(default = None)]
    pub class: Option<&'static str>,

    #[props(default = None)]
    pub style: Option<&'static str>,

    #[props(default = None)]
    pub width: Option<&'static str>,

    #[props(default = None)]
    pub height: Option<&'static str>,

    #[props(default = None)]
    pub fill: Option<&'static str>,

    #[props(default = None)]
    pub stroke: Option<&'static str>,

    #[props(default = Some("0 0 512 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn TorBrowser(props: TorBrowserProps) -> Element {
    rsx! {
        svg {
            class: props.class,
            style: props.style,
            height: props.height,
            width: props.width,
            view_box: props.view_box.unwrap_or("0 0 16 16"),
            xmlns: props.xmlns.unwrap_or("http://www.w3.org/2000/svg"),
            fill: props.fill.unwrap_or("currentColor"),
            stroke: props.stroke,

            if let Some(title_text) = props.title {
                title {{ title_text }}
            }

                        path {
        d: "M256.5 465.4l0-31c98.3-.3 177.9-80 177.9-178.4S354.8 77.9 256.5 77.6l0-31C372 46.8 465.4 140.5 465.4 256S372 465.2 256.5 465.4zm0-108.6c55.4-.3 100.3-45.3 100.3-100.8S312 155.5 256.5 155.2l0-31c72.6 .3 131.4 59.2 131.4 131.8S329.1 387.6 256.5 387.8l0-31zm0-155.1c29.7 .3 53.8 24.5 53.8 54.3s-24 54-53.8 54.3l0-108.6zM0 256a256 256 0 1 0 512 0 256 256 0 1 0 -512 0z",
            }
        }
    }
}
