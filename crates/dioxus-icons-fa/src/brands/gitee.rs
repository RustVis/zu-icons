// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct GiteeProps {
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

pub fn Gitee(props: GiteeProps) -> Element {
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
        d: "M256 512a256 256 0 1 1 0-512 256 256 0 1 1 0 512zM385.6 113.8l0 0-177 0c-52.4 0-94.8 42.4-94.8 94.8l0 177c0 7 5.7 12.6 12.6 12.6l186.5 0c47.1 0 85.3-38.2 85.3-85.3l0-72.7c0-7-5.7-12.6-12.6-12.6l-145.4 0c-7 0-12.6 5.7-12.6 12.6l0 31.6c0 6.6 5.1 12.1 11.6 12.6l1 0 88.5 0c6.6 0 12.1 5.1 12.6 11.6l0 1 0 6.3c0 20.9-17 37.9-37.9 37.9l-120.1 0c-7 0-12.6-5.7-12.6-12.6l0-120.1c0-20.4 16.1-37.1 36.4-37.9l1.6 0 177 0c7 0 12.6-5.7 12.6-12.6l0-31.6c0-7-5.6-12.6-12.6-12.6z",
            }
        }
    }
}
