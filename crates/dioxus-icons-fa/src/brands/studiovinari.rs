// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct StudiovinariProps {
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

pub fn Studiovinari(props: StudiovinariProps) -> Element {
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
        d: "M480.7 187.7l4.2 28 0 28-25.1 44.1-39.8 78.4-56.1 67.5-79.1 37.8-17.7 24.5-7.7 12-9.6 4s17.3-63.6 19.4-63.6 20.3 .7 20.3 .7l66.7-38.6-92.5 26.1-55.9 36.8-22.8 28-6.6 1.4 20.8-73.6 6.9-5.5 20.7 12.9 88.3-45.2 56.8-51.5 14.8-68.4-125.4 23.3 15.2-18.2-173.4-53.3 81.9-10.5-166-122.9 114.9 18.1-101.3-108 252.9 126.6-31.5-38 124.4 74.4-143.3-99 18.7 38.4-49.6-18.1-45.5-84.3 194.6 122-42.9-55.8 108 96.4 12-8.9-21-16.4 4.2-37.8 37.8-10.4 29.2 24.7 11.5 4.2-7 6.2 8.5 12-13.1 7.4-10.3 20.2 10.5 23.9z",
            }
        }
    }
}
