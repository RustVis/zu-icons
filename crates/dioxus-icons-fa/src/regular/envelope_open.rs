// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct EnvelopeOpenProps {
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

pub fn EnvelopeOpen(props: EnvelopeOpenProps) -> Element {
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
        d: "M512 416c0 35.3-28.5 64-63.9 64L64 480c-35.4 0-64-28.7-64-64L0 164c.1-15.5 7.8-30 20.5-38.8L206-2.7c30.1-20.7 69.8-20.7 99.9 0L491.5 125.2c12.8 8.8 20.4 23.3 20.5 38.8l0 252zM64 432l384.1 0c8.8 0 15.9-7.1 15.9-16l0-191.7-154.8 117.4c-31.4 23.9-74.9 23.9-106.4 0L48 224.3 48 416c0 8.9 7.2 16 16 16zM463.6 164.4L278.7 36.8c-13.7-9.4-31.7-9.4-45.4 0L48.4 164.4 231.8 303.5c14.3 10.8 34.1 10.8 48.4 0L463.6 164.4z",
            }
        }
    }
}
