// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BaconProps {
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

    #[props(default = Some("0 0 576 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn Bacon(props: BaconProps) -> Element {
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
        d: "M557 96.7c14.4 14.4 12.4 38.4-4.3 50.2l-64.6 45.7c-43.7 30.9-79.2 71.9-103.4 119.6l-25.3 49.8c-25.1 49.3-62.1 91.5-107.8 122.6l-74.1 50.6c-13.1 8.9-30.7 7.3-41.8-3.9l-44.9-44.9 86.5-66.5c42.3-32.5 76.7-74.3 100.6-122l24.5-49.1c24.5-49 61.8-90.6 107.9-120.2l108.7-69.9 38 38zM484.2 23.9L384.3 88.2c-53.4 34.3-96.5 82.4-124.9 139.1l-24.5 49.1c-20.6 41.3-50.3 77.3-86.9 105.4l-91.4 70.3-36.9-36.9c-14.4-14.4-12.4-38.4 4.3-50.2l64.6-45.7c43.7-30.9 79.2-71.9 103.4-119.6l25.3-49.8C242.3 100.8 279.3 58.6 325 27.4l74.1-50.6c13.1-8.9 30.6-7.3 41.8 3.9l43.3 43.3z",
            }
        }
    }
}
