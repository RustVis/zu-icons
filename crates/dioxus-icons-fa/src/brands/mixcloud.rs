// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct MixcloudProps {
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

    #[props(default = Some("0 0 640 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Mixcloud(props: MixcloudProps) -> Element {
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
        d: "M213 346.6l-33.2 0 0-151.5 6.2-21.6-10.7 0-38.1 173.1-61 0-38.4-173.1-10.5 0 5.9 21.6 0 151.5-33.2 0 0-181.6 65.7 0 36.6 173.1 8.5 0 36.6-173.1 65.7 0 0 181.6zm331.5-63l-86 62.1 0-38.1 72.9-51.8-72.9-51.8 0-38.1 86 62.3 9.3 0 86.3-62.3 0 38.1-73.1 51.8 73.1 51.8 0 38.1-86.3-62.1-9.3 0zM430.2 272.3l-182 0 0-33.1 182 0 0 33.1z",
            }
        }
    }
}
