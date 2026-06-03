// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CloudsmithProps {
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

pub fn Cloudsmith(props: CloudsmithProps) -> Element {
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
        d: "M512 227.6L512 284.5 284.4 512 227.6 512 0 284.4 0 227.6 227.6 0 284.5 0 512 227.6zm-256 162c17.8 .5 35.6-2.6 52.2-9.1s31.8-16.2 44.6-28.7 23-27.3 29.9-43.8 10.5-34.1 10.5-52-3.6-35.5-10.5-52-17.1-31.3-29.9-43.8-28-22.2-44.6-28.7-34.4-9.6-52.2-9.1c-17.8-.5-35.6 2.6-52.2 9.1s-31.8 16.3-44.6 28.7-23 27.3-29.9 43.8-10.5 34.1-10.5 52 3.6 35.5 10.5 52 17.1 31.3 29.9 43.8 28 22.2 44.6 28.7 34.4 9.6 52.2 9.1z",
            }
        }
    }
}
