// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct BridgeWaterProps {
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

pub fn BridgeWater(props: BridgeWaterProps) -> Element {
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
        d: "M64 64l512 0 0 112c-37.6 9.4-64 43.2-64 82l0 76.4c-21-9.7-43.5-14.5-66-14.4-10 .1-20.1 1.1-30 3.1l0-35.1c0-53-43-96-96-96s-96 43-96 96l0 32.7c-5.3-.5-10.7-.8-16-.7-27.7 .2-55.4 7.8-80 23l0-85c0-38.8-26.4-72.6-64-82L64 64zM403.4 444.1C379.1 462.3 351.1 480 320 480s-59.1-17.7-83.4-35.9c-21.3-16.1-49.9-16.1-71.2 0-23.8 17.9-54.1 35.5-88.1 35.3-20.4-.1-40.7-6.7-59.8-21.1-10.6-8-12.7-23-4.7-33.6s23-12.7 33.6-4.7c11.3 8.5 21.6 11.4 31.2 11.5 17.6 .1 37.3-9.4 58.9-25.7 38.4-29 90.5-29 129 0 24 18.1 40.7 26.3 54.5 26.3s30.5-8.2 54.5-26.3c38.4-29 90.5-29 129 0 16.9 12.7 32.9 21.5 47.8 24.6 13.7 2.8 27.4 .9 42.3-10.3 10.6-8 25.6-5.9 33.6 4.7s5.9 25.6-4.7 33.6c-26.4 19.9-54.2 24.4-80.7 19.1-25.3-5.1-48.1-18.9-67.2-33.3-21.3-16.1-49.9-16.1-71.2 0z",
            }
        }
    }
}
