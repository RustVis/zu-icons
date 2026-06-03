// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DiceD6Props {
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

pub fn DiceD6(props: DiceD6Props) -> Element {
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
        d: "M224.4 8.3C244-2.8 268-2.8 287.6 8.3l176 99.7c20 11.4 32.4 32.6 32.4 55.7l0 197.4c0 23-12.4 44.3-32.4 55.7l-176 99.7c-19.6 11.1-43.6 11.1-63.1 0l-176-99.7C28.4 405.5 16 384.2 16 361.2l0-197.4c0-23 12.4-44.3 32.4-55.7l176-99.7zM102.6 155.6c-8.8-3.1-18.8 .3-23.8 8.6s-3.2 18.7 3.6 25l3.2 2.4 150.2 90.2 0 148.7c0 11 9 20 20 20 11 0 20-9 20-20l0-148.7 150.3-90.2c9.5-5.7 12.6-18 6.9-27.4s-18-12.5-27.4-6.9l-149.7 89.8-149.7-89.8-3.7-1.7z",
            }
        }
    }
}
