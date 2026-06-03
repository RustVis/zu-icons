// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SocksProps {
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

pub fn Socks(props: SocksProps) -> Element {
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
        d: "M252.8 0L176 0c-26.5 0-48 21.5-48 48l0 16 112 0 0-16c0-17.5 4.7-33.9 12.8-48zM128 112l0 128c0 20.1-9.5 39.1-25.6 51.2l-64 48c-24.2 18.1-38.4 46.6-38.4 76.8 0 53 43 96 96 96 15.4 0 30.5-3.7 44-10.7-17.6-23.9-28-53.4-28-85.3 0-45.3 21.3-88 57.6-115.2l64-48c4-3 6.4-7.8 6.4-12.8l0-128-112 0zm160 0l0 128c0 20.1-9.5 39.1-25.6 51.2l-64 48c-24.2 18.1-38.4 46.6-38.4 76.8 0 53 43 96 96 96 20.8 0 41-6.7 57.6-19.2l115.2-86.4C461 382.2 480 344.3 480 304l0-192-192 0zM480 64l0-16c0-26.5-21.5-48-48-48L336 0c-26.5 0-48 21.5-48 48l0 16 192 0z",
            }
        }
    }
}
