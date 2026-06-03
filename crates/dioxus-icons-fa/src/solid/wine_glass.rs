// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct WineGlassProps {
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

pub fn WineGlass(props: WineGlassProps) -> Element {
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
        d: "M32.6 25.7C35.6 10.8 48.7 0 64 0L256 0c15.3 0 28.4 10.8 31.4 25.7L316.8 173c2.1 10.5 3.2 21.2 3.2 32l0 3c0 77.4-55 142-128 156.8l0 115.2 64 0c17.7 0 32 14.3 32 32s-14.3 32-32 32L64 544c-17.7 0-32-14.3-32-32s14.3-32 32-32l64 0 0-115.2C55 350 0 285.4 0 208l0-3c0-10.7 1.1-21.4 3.2-32L32.6 25.7zM77.4 128l165.1 0-12.8-64-139.5 0-12.8 64z",
            }
        }
    }
}
