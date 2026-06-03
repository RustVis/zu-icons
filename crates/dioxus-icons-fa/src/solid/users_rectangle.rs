// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct UsersRectangleProps {
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

pub fn UsersRectangle(props: UsersRectangleProps) -> Element {
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
        d: "M64 32C28.7 32 0 60.7 0 96L0 416c0 35.3 28.7 64 64 64l448 0c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64L64 32zm224 72a56 56 0 1 1 0 112 56 56 0 1 1 0-112zm0 152c53 0 96 43 96 96l0 24c0 13.3-10.7 24-24 24l-144 0c-13.3 0-24-10.7-24-24l0-24c0-53 43-96 96-96zm96-64a48 48 0 1 1 96 0 48 48 0 1 1 -96 0zM168 272.3c-15.2 22.8-24 50.2-24 79.7l0 24c0 8.4 1.4 16.5 4.1 24l-46.8 0C89.6 400 80 390.4 80 378.7L80 368c0-50.3 38.7-91.6 88-95.7zM427.9 400c2.7-7.5 4.1-15.6 4.1-24l0-24c0-29.5-8.8-56.9-24-79.7 49.3 4.1 88 45.3 88 95.7l0 10.7c0 11.8-9.6 21.3-21.3 21.3l-46.8 0zM96 192a48 48 0 1 1 96 0 48 48 0 1 1 -96 0z",
            }
        }
    }
}
