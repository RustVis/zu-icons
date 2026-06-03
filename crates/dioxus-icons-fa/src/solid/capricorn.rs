// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CapricornProps {
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

pub fn Capricorn(props: CapricornProps) -> Element {
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
        d: "M240 32c79.5 0 144 64.5 144 144l0 65.2c18.8-10.9 40.7-17.2 64-17.2 70.7 0 128 57.3 128 128S518.7 480 448 480c-35.8 0-68.2-14.7-91.4-38.4-28.8 42.5-77.4 70.4-132.6 70.4-17.7 0-32-14.3-32-32s14.3-32 32-32c53 0 96-43 96-96l0-176c0-44.2-35.8-80-80-80s-80 35.8-80 80l0 208c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-224c0-35.3-28.7-64-64-64-17.7 0-32-14.3-32-32S14.3 32 32 32C72.6 32 108.8 51 132.2 80.5 158.6 50.7 197.1 32 240 32zM448 288a64 64 0 1 0 0 128 64 64 0 1 0 0-128z",
            }
        }
    }
}
