// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct IntercomProps {
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

pub fn Intercom(props: IntercomProps) -> Element {
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
        d: "M392 32L56 32C25.1 32 0 57.1 0 88L0 424c0 30.9 25.1 56 56 56l336 0c30.9 0 56-25.1 56-56l0-336c0-30.9-25.1-56-56-56zM283.7 114.1c0-19.8 29.9-19.8 29.9 0l0 199.5c0 19.8-29.9 19.8-29.9 0l0-199.5zm-74.6-7.5c0-19.8 29.9-19.8 29.9 0l0 216.5c0 19.8-29.9 19.8-29.9 0l0-216.5zm-74.7 7.5c0-19.8 29.9-19.8 29.9 0l0 199.5c0 19.8-29.9 19.8-29.9 0l0-199.5zM59.7 144c0-19.8 29.9-19.8 29.9 0l0 134.3c0 19.8-29.9 19.8-29.9 0l0-134.3zM383.1 371.8c-72.8 63-241.7 65.4-318.1 0-15-12.8 4.4-35.5 19.4-22.7 65.9 55.3 216.1 53.9 279.3 0 14.9-12.9 34.3 9.8 19.4 22.7zm5.2-93.5c0 19.8-29.9 19.8-29.9 0l0-134.3c0-19.8 29.9-19.8 29.9 0l0 134.3z",
            }
        }
    }
}
