// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct TanakhProps {
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

pub fn Tanakh(props: TanakhProps) -> Element {
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
        d: "M352 512L32 512c-17.7 0-32-14.3-32-32s14.3-32 32-32l0-66.7C13.4 374.7 0 356.9 0 336L0 48C0 21.5 21.5 0 48 0L352 0c53 0 96 43 96 96l0 320c0 53-43 96-96 96zm32-96c0-17.7-14.3-32-32-32l-256 0 0 64 256 0c17.7 0 32-14.3 32-32zM113.9 229c-1.2 2-1.9 4.2-1.9 6.6 0 6.9 5.6 12.5 12.5 12.5l56.9 0 30.5 49.2c2.6 4.2 7.2 6.8 12.2 6.8s9.6-2.6 12.2-6.8l30.5-49.2 56.9 0c6.9 0 12.5-5.6 12.5-12.5 0-2.3-.6-4.6-1.9-6.6l-27.8-45 27.8-45c1.2-2 1.9-4.2 1.9-6.6 0-6.9-5.6-12.5-12.5-12.5l-56.9 0-30.5-49.2C233.6 66.6 229 64 224 64s-9.6 2.6-12.2 6.8l-30.5 49.2-56.9 0c-6.9 0-12.5 5.6-12.5 12.5 0 2.3 .6 4.6 1.9 6.6l27.8 45-27.8 45z",
            }
        }
    }
}
