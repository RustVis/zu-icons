// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SmokingBanProps {
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

pub fn SmokingBan(props: SmokingBanProps) -> Element {
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
        d: "M99.5 144.8l79.2 79.2-50.7 0c-17.7 0-32 14.3-32 32l0 32c0 17.7 14.3 32 32 32l146.7 0 92.5 92.5c-31.4 22.4-69.8 35.5-111.2 35.5-106 0-192-86-192-192 0-41.5 13.1-79.9 35.5-111.2zM333.3 288l-32-32 82.7 0 0 32-50.7 0zm32 32l18.7 0c17.7 0 32-14.3 32-32l0-32c0-17.7-14.3-32-32-32L269.3 224 144.8 99.5c31.4-22.4 69.8-35.5 111.2-35.5 106 0 192 86 192 192 0 41.5-13.1 79.9-35.5 111.2L365.3 320zM256 512a256 256 0 1 0 0-512 256 256 0 1 0 0 512zM272 96c-8.8 0-16 7.2-16 16 0 26.5 21.5 48 48 48l32 0c8.8 0 16 7.2 16 16s7.2 16 16 16 16-7.2 16-16c0-26.5-21.5-48-48-48l-32 0c-8.8 0-16-7.2-16-16s-7.2-16-16-16z",
            }
        }
    }
}
