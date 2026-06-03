// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DribbbleSquareProps {
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

pub fn DribbbleSquare(props: DribbbleSquareProps) -> Element {
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
        d: "M165.9 132.5c-38.3 18-66.8 53.3-75.7 95.7 6.1 .1 62.4 .3 126.4-16.7-22.7-40.2-47.1-74.1-50.7-79zm26.1-9.1c3.8 5.1 28.6 38.9 51 80 48.6-18.3 69.1-45.9 71.6-49.4-33.6-29.8-79.3-41.1-122.6-30.6zM277.4 382c-2-12-10-53.8-29.2-103.6-55.1 18.8-93.8 56.4-108.1 85.6 40.5 31.6 93.3 36.7 137.3 18zM227.8 232.6C159.6 253 93.4 252.2 87.4 252l0 4.2c0 35.1 13.3 67.1 35.1 91.4 22.2-37.9 67.1-77.9 116.5-91.8-3.4-7.8-7.2-15.5-11.1-23.2l-.1 0zm72.5 136.9c30.7-20.7 52.5-53.6 58.6-91.6-4.6-1.5-42.3-12.7-85.1-5.8 17.9 49.1 25.1 89.1 26.5 97.4zm-34.8-119c45.5-5.7 90.7 3.4 95.2 4.4-.3-32.3-11.8-61.9-30.9-85.1-2.9 3.9-25.8 33.2-76.3 53.9 4.8 9.8 8.3 17.8 12 26.8zM384 32L64 32C28.7 32 0 60.7 0 96L0 416c0 35.3 28.7 64 64 64l320 0c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64zM224 96a160 160 0 1 1 0 320 160 160 0 1 1 0-320z",
            }
        }
    }
}
