// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct HandcuffsProps {
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

pub fn Handcuffs(props: HandcuffsProps) -> Element {
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
        d: "M320-32c0-17.7-14.3-32-32-32s-32 14.3-32 32 14.3 32 32 32 32-14.3 32-32zM192 64a32 32 0 1 0 0-64 32 32 0 1 0 0 64zM152 96c-13.3 0-24 10.7-24 24l0 16c0 1 .1 1.9 .2 2.9-74.7 26.3-128.2 97.5-128.2 181.1 0 106 86 192 192 192s192-86 192-192c0-83.7-53.5-154.8-128.2-181.1 .1-.9 .2-1.9 .2-2.9l0-16c0-13.3-10.7-24-24-24l-80 0zM64 320a128 128 0 1 1 256 0 128 128 0 1 1 -256 0zm448 0c0 66.9-51.3 121.8-116.6 127.5-14.3 22.8-32.4 43.1-53.4 59.9 13.5 3 27.6 4.6 42 4.6 106 0 192-86 192-192 0-83.7-53.5-154.8-128.2-181.1 .1-.9 .2-1.9 .2-2.9l0-16c0-13.3-10.7-24-24-24l-80 0c-12.3 0-22.4 9.2-23.8 21.1 30.3 19.2 56.1 45 75.2 75.4 65.4 5.8 116.6 60.6 116.6 127.5zM384 64a32 32 0 1 0 0-64 32 32 0 1 0 0 64z",
            }
        }
    }
}
