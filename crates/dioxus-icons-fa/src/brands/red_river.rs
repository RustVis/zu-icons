// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct RedRiverProps {
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

    #[props(default = Some("0 0 448 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn RedRiver(props: RedRiverProps) -> Element {
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
        d: "M353.2 32L94.8 32C42.4 32 0 74.4 0 126.8L0 385.2C0 437.6 42.4 480 94.8 480l258.4 0c52.4 0 94.8-42.4 94.8-94.8l0-258.4C448 74.4 405.6 32 353.2 32zM144.9 200.9l0 56.3c0 27-21.9 48.9-48.9 48.9l0-154.2c0-13.2 10.7-23.9 23.9-23.9l154.2 0c0 27-21.9 48.9-48.9 48.9l-56.3 0c-12.3-.6-24.6 11.6-24 24zm176.3 72l-56.3 0c-12.3-.6-24.6 11.6-24 24l0 56.3c0 27-21.9 48.9-48.9 48.9l0-154.2c0-13.2 10.7-23.9 23.9-23.9l154.2 0c0 27-21.9 48.9-48.9 48.9z",
            }
        }
    }
}
