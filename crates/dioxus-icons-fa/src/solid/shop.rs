// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ShopProps {
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

pub fn Shop(props: ShopProps) -> Element {
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
        d: "M21.5 181.1L78.3 67.4C89.2 45.7 111.3 32 135.6 32l304.9 0c24.2 0 46.4 13.7 57.2 35.4l56.8 113.7c3.6 7.2 5.5 15.1 5.5 23.2 0 27.3-21.2 49.7-48 51.6L512 448c0 17.7-14.3 32-32 32s-32-14.3-32-32l0-192-96 0 0 176c0 26.5-21.5 48-48 48l-192 0c-26.5 0-48-21.5-48-48l0-176.1c-26.8-1.9-48-24.3-48-51.6 0-8 1.9-16 5.5-23.2zM128 256l0 112c0 8.8 7.2 16 16 16l128 0c8.8 0 16-7.2 16-16l0-112-160 0z",
            }
        }
    }
}
