// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct WineGlassEmptyProps {
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

    #[props(default = Some("0 0 320 512"))]
    pub view_box: Option<&'static str>,

    #[props(default = None)]
    pub xmlns: Option<&'static str>,
}

pub fn WineGlassEmpty(props: WineGlassEmptyProps) -> Element {
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
        d: "M64 0C48.7 0 35.6 10.8 32.6 25.7L3.2 173C1.1 183.5 0 194.2 0 205l0 3c0 77.4 55 142 128 156.8l0 115.2-64 0c-17.7 0-32 14.3-32 32s14.3 32 32 32l192 0c17.7 0 32-14.3 32-32s-14.3-32-32-32l-64 0 0-115.2C265 350 320 285.4 320 208l0-3c0-10.7-1.1-21.4-3.2-32L287.4 25.7C284.4 10.8 271.3 0 256 0L64 0zm1.9 185.6L90.2 64 229.8 64 254.1 185.6c1.3 6.4 1.9 12.9 1.9 19.4l0 3c0 53-43 96-96 96s-96-43-96-96l0-3c0-6.5 .6-13 1.9-19.4z",
            }
        }
    }
}
