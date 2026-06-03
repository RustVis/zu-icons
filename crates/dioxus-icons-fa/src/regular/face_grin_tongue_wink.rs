// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct FaceGrinTongueWinkProps {
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

pub fn FaceGrinTongueWink(props: FaceGrinTongueWinkProps) -> Element {
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
        d: "M366.9 432c.8-5.2 1.1-10.6 1.1-16l0-53.5c10.2-12.6 18.3-26.9 23.8-42.5 4.1-11.6-7.8-21.4-19.6-17.8-34.8 10.6-74.3 16.6-116.3 16.6-41.9 0-81.4-6-116.1-16.5-11.8-3.6-23.7 6.1-19.6 17.8 5.5 15.5 13.6 29.9 23.8 42.4l0 53.5c0 5.4 .4 10.8 1.1 16-58.4-36.8-97.1-101.9-97.1-176 0-114.9 93.1-208 208-208s208 93.1 208 208c0 74.1-38.8 139.2-97.1 176zm-38.8 69.7C434.4 470.6 512 372.3 512 256 512 114.6 397.4 0 256 0S0 114.6 0 256C0 372.3 77.6 470.6 183.9 501.7 203.4 518.1 228.5 528 256 528s52.6-9.9 72.1-26.3zM320 378.6l0 37.4c0 35.3-28.7 64-64 64s-64-28.7-64-64l0-37.4c0-14.7 11.9-26.6 26.6-26.6l2 0c11.3 0 21.1 7.9 23.6 18.9 2.8 12.6 20.8 12.6 23.6 0 2.5-11.1 12.3-18.9 23.6-18.9l2 0c14.7 0 26.6 11.9 26.6 26.6zM132 232c0-11 9-20 20-20l16 0c11 0 20 9 20 20s9 20 20 20 20-9 20-20c0-33.1-26.9-60-60-60l-16 0c-33.1 0-60 26.9-60 60 0 11 9 20 20 20s20-9 20-20zm228.4-24a24 24 0 1 0 -48 0 24 24 0 1 0 48 0zM288 208a48 48 0 1 1 96 0 48 48 0 1 1 -96 0zm128 0a80 80 0 1 0 -160 0 80 80 0 1 0 160 0z",
            }
        }
    }
}
