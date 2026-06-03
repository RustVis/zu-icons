// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct StubberProps {
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

pub fn Stubber(props: StubberProps) -> Element {
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
        d: "M136.5 294.2l58.8 22.9c9.1-36.8 25.4-61.1 55-61.1 49.4 0 71.4 63.6 142.4 63.6 15.6 0 35.9-2.8 55.3-13.3l0 61.7c0 61.8-50.4 112-112.3 112L0 480 41.8 424 0 368 41.7 312 0 256.1 41.8 200.1 0 144.1 41.8 88 0 32 335.7 32C397.6 32 448 82.3 448 144.1l0 51.3c-9.2 36.3-25.9 60.6-55 60.6-49.6 0-71.6-63.5-142.4-63.5-35.9 0-95.2 14.6-114.1 101.6l0 .1z",
            }
        }
    }
}
