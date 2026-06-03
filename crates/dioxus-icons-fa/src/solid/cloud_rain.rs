// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CloudRainProps {
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

pub fn CloudRain(props: CloudRainProps) -> Element {
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
        d: "M96 320c-53 0-96-43-96-96 0-42.5 27.6-78.6 65.9-91.2-1.3-6.7-1.9-13.7-1.9-20.8 0-61.9 50.1-112 112-112 43.1 0 80.5 24.3 99.2 60 14.7-17.1 36.5-28 60.8-28 44.2 0 80 35.8 80 80 0 5.5-.6 10.8-1.6 16 .5 0 1.1 0 1.6 0 53 0 96 43 96 96s-43 96-96 96L96 320zm1.6 68.2c1.1-2.5 3.6-4.2 6.4-4.2s5.3 1.6 6.4 4.2l30.2 68.2c2.2 5.1 3.4 10.5 3.4 16 0 21.9-18.1 39.6-40 39.6s-40-17.7-40-39.6c0-5.5 1.2-11 3.4-16l30.2-68.2zm152 0c1.1-2.5 3.6-4.2 6.4-4.2s5.3 1.6 6.4 4.2l30.2 68.2c2.2 5.1 3.4 10.5 3.4 16 0 21.9-18.1 39.6-40 39.6s-40-17.7-40-39.6c0-5.5 1.2-11 3.4-16l30.2-68.2zm121.8 68.2l30.2-68.2c1.1-2.5 3.6-4.2 6.4-4.2s5.3 1.6 6.4 4.2l30.2 68.2c2.2 5.1 3.4 10.5 3.4 16 0 21.9-18.1 39.6-40 39.6s-40-17.7-40-39.6c0-5.5 1.2-11 3.4-16z",
            }
        }
    }
}
