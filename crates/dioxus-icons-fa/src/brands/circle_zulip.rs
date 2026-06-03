// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct CircleZulipProps {
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

pub fn CircleZulip(props: CircleZulipProps) -> Element {
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
        d: "M256 512a256 256 0 1 1 0-512 256 256 0 1 1 0 512zM243.5 240c1.2-1.9-.9-4.2-2.6-2.8l-89 79.5c-9.2 7.2-15.2 19.1-15.2 32.5 0 21.9 16 39.8 35.7 39.8l167.3 0c19.6 0 35.7-17.9 35.7-39.8 0-21.9-16.1-39.7-35.7-39.8l-126.6 0c-1.9 0-3-2.3-2.1-4.1L243.5 240zM172.4 123.1c-19.6 0-35.7 17.9-35.7 39.8s16 39.8 35.7 39.8l126.6 0c1.9 0 3.1 2.2 2.2 4.1L268.5 272c-1.2 1.9 .9 4.2 2.6 2.8l89-79.5c9.2-7.2 15.2-19.1 15.2-32.5 0-21.9-16-39.7-35.7-39.8l-167.3 0z",
            }
        }
    }
}
