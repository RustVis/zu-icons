// Copyright (c) 2026 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct ArrowAltCircleLeftProps {
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

pub fn ArrowAltCircleLeft(props: ArrowAltCircleLeftProps) -> Element {
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
        d: "M48 256a208 208 0 1 1 416 0 208 208 0 1 1 -416 0zm464 0a256 256 0 1 0 -512 0 256 256 0 1 0 512 0zM124.7 244.7c-6.2 6.2-6.2 16.4 0 22.6l104 104c4.6 4.6 11.5 5.9 17.4 3.5s9.9-8.3 9.9-14.8l0-72 104 0c13.3 0 24-10.7 24-24l0-16c0-13.3-10.7-24-24-24l-104 0 0-72c0-6.5-3.9-12.3-9.9-14.8s-12.9-1.1-17.4 3.5l-104 104z",
            }
        }
    }
}
